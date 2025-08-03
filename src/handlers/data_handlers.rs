// src/handlers/data_handlers.rs
// Handler para consulta de dados de vendas do FC PostgreSQL

use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::Pool;
use serde_json::json;
use chrono::NaiveDate;

use crate::models::FiltrosVenda;

/// Handler para obter dados de vendas usando a query fornecida
pub async fn get_vendas(
    pool: web::Data<Pool>,
    filtros: web::Query<FiltrosVenda>,
) -> Result<HttpResponse> {
    log::info!("Consultando vendas com filtros: {:?}", filtros);

    // USANDO O MESMO PADRÃO QUE FUNCIONA EM test_postgres
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro no pool de conexões",
                "error": e.to_string()
            })));
        }
    };

    // Query exata conforme fornecida
    let mut query = String::from(
        "SELECT 
            cab.companygroupname,
            cfg.cnpj,
            cab.cdfil,
            emp.descrfil,
            cab.nrcpm,
            cab.dtpagefe,
            cab.dteminfce,
            cab.cdcli,
            cli.nomecli,
            cab.cdfunre,
            ven.nomefun,
            it.itemid,
            it.cdpro,
            pr.descrprd,
            pr.setor,
            it.quant,
            it.pruni,
            it.vrtot,
            it.vrdsc,
            it.vrrcb,
            it.prcusto,
            it.prcompra
        FROM FC14000 as cab"
    );

    // Joins conforme query fornecida
    query.push_str(" INNER JOIN (SELECT company_id, cnpj, companygroupname FROM company_config) cfg");
    query.push_str(" ON cab.company_id = cfg.company_id AND cab.companygroupname = cfg.companygroupname");
    
    query.push_str(" LEFT JOIN (SELECT company_id, cdcli, nomecli FROM fc07000) cli");
    query.push_str(" ON cab.company_id = cli.company_id AND cab.cdcli = cli.cdcli");
    
    query.push_str(" INNER JOIN (SELECT company_id, cdfun, nomefun FROM fc08000 GROUP BY company_id, cdfun, nomefun) ven");
    query.push_str(" ON cab.company_id = ven.company_id AND cab.cdfunre = ven.cdfun");
    
    query.push_str(" INNER JOIN (SELECT company_id, cdfil, nrcpm, itemid, cdpro, quant,");
    query.push_str(" CAST(pruni as numeric) pruni, CAST(vrtot as numeric) vrtot,");
    query.push_str(" CAST(vrdsc as numeric) vrdsc,");
    query.push_str(" ROUND(CAST((vrtot+vrtxav) - (vrdsc + vrdscv) as numeric),2) vrrcb,");
    query.push_str(" prcusto, prcompra FROM fc14100) it");
    query.push_str(" ON it.company_id = cab.company_id AND it.cdfil = cab.cdfil AND it.nrcpm = cab.nrcpm");
    
    query.push_str(" LEFT JOIN (SELECT company_id, cdpro, descrprd, setor FROM fc03000 pr WHERE 1=1) pr");
    query.push_str(" ON it.company_id = pr.company_id AND it.cdpro = pr.cdpro");
    
    query.push_str(" INNER JOIN (SELECT company_id, cdfil, descrfil FROM companies) emp");
    query.push_str(" ON cab.company_id = emp.company_id AND cab.cdfil = emp.cdfil");
    
    query.push_str(" INNER JOIN (SELECT company_id, company_name FROM company_config) cc");
    query.push_str(" ON cab.company_id = cc.company_id");
    
    query.push_str(" WHERE pr.cdpro IS NOT NULL");

    // Aplicar filtros
    let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Sync + Send>> = vec![];
    let mut param_count = 0;

    if let Some(data_inicio) = &filtros.data_inicio {
        // Converter string para NaiveDate
        if let Ok(data) = NaiveDate::parse_from_str(data_inicio, "%Y-%m-%d") {
            param_count += 1;
            query.push_str(&format!(" AND cab.dtpagefe >= ${}", param_count));
            params.push(Box::new(data));
        }
    }

    if let Some(data_fim) = &filtros.data_fim {
        // Converter string para NaiveDate
        if let Ok(data) = NaiveDate::parse_from_str(data_fim, "%Y-%m-%d") {
            param_count += 1;
            query.push_str(&format!(" AND cab.dtpagefe <= ${}", param_count));
            params.push(Box::new(data));
        }
    }

    if let Some(empresa) = &filtros.empresa {
        param_count += 1;
        query.push_str(&format!(" AND cab.companygroupname = ${}", param_count));
        params.push(Box::new(empresa.clone()));
    }

    if let Some(filial) = &filtros.filial {
        param_count += 1;
        query.push_str(&format!(" AND cab.cdfil = ${}", param_count));
        params.push(Box::new(filial.clone()));
    }

    if let Some(vendedor) = &filtros.vendedor {
        param_count += 1;
        query.push_str(&format!(" AND cab.cdfunre = ${}", param_count));
        params.push(Box::new(vendedor.clone()));
    }

    if let Some(produto) = &filtros.produto {
        param_count += 1;
        query.push_str(&format!(" AND pr.descrprd ILIKE ${}", param_count));
        params.push(Box::new(format!("%{}%", produto)));
    }

    // Ordenação conforme query original
    query.push_str(" ORDER BY cab.dtpagefe, cab.company_id");

    // Adicionar limite se especificado
    if let Some(limite) = filtros.limite {
        query.push_str(&format!(" LIMIT {}", limite));
    }

    // Executar query
    let stmt = match client.prepare(&query).await {
        Ok(stmt) => stmt,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro ao preparar query",
                "error": e.to_string()
            })));
        }
    };

    // Converter para referências simples
    let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params
        .iter()
        .map(|b| b.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
        .collect();

    let rows = match client.query(&stmt, &param_refs[..]).await {
        Ok(rows) => rows,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "success": false,
                "message": "Erro ao executar query",
                "error": e.to_string()
            })));
        }
    };

    // Converter rows para JSON de forma dinâmica
    let mut results = Vec::new();
    
    for row in &rows {
        let mut item = serde_json::Map::new();
        
        // Campos da query
        item.insert("companygroupname".to_string(), json!(row.get::<_, Option<String>>(0)));
        item.insert("cnpj".to_string(), json!(row.get::<_, Option<String>>(1)));
        item.insert("cdfil".to_string(), json!(row.get::<_, Option<i32>>(2)));
        item.insert("descrfil".to_string(), json!(row.get::<_, Option<String>>(3)));
        item.insert("nrcpm".to_string(), json!(row.get::<_, Option<i64>>(4)));
        item.insert("dtpagefe".to_string(), json!(row.get::<_, Option<NaiveDate>>(5)));
        item.insert("dteminfce".to_string(), json!(row.get::<_, Option<NaiveDate>>(6)));
        item.insert("cdcli".to_string(), json!(row.get::<_, Option<i32>>(7)));
        item.insert("nomecli".to_string(), json!(row.get::<_, Option<String>>(8)));
        item.insert("cdfunre".to_string(), json!(row.get::<_, Option<i32>>(9)));
        item.insert("nomefun".to_string(), json!(row.get::<_, Option<String>>(10)));
        item.insert("itemid".to_string(), json!(row.get::<_, Option<i32>>(11)));
        item.insert("cdpro".to_string(), json!(row.get::<_, Option<i32>>(12)));
        item.insert("descrprd".to_string(), json!(row.get::<_, Option<String>>(13)));
        item.insert("setor".to_string(), json!(row.get::<_, Option<String>>(14)));
        item.insert("quant".to_string(), json!(row.get::<_, Option<f64>>(15)));
        item.insert("pruni".to_string(), json!(row.get::<_, Option<f64>>(16)));
        item.insert("vrtot".to_string(), json!(row.get::<_, Option<f64>>(17)));
        item.insert("vrdsc".to_string(), json!(row.get::<_, Option<f64>>(18)));
        item.insert("vrrcb".to_string(), json!(row.get::<_, Option<f64>>(19)));
        item.insert("prcusto".to_string(), json!(row.get::<_, Option<f64>>(20)));
        item.insert("prcompra".to_string(), json!(row.get::<_, Option<f64>>(21)));
        
        results.push(json!(item));
    }

    log::info!("Retornando {} registros", results.len());

    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "data": results,
        "total": results.len()
    })))
}

/// Handler para obter vendas detalhadas - usa a mesma query
pub async fn get_vendas_detalhadas(
    pool: web::Data<Pool>,
    filtros: web::Query<FiltrosVenda>,
) -> Result<HttpResponse> {
    // Chama o mesmo handler pois é a mesma query
    get_vendas(pool, filtros).await
}

/// Handler para debug - mostra a query que seria executada
pub async fn debug_query(
    filtros: web::Query<FiltrosVenda>,
) -> Result<HttpResponse> {
    // Construir a mesma query
    let mut query = String::from(
        "SELECT 
            cab.companygroupname,
            cfg.cnpj,
            cab.cdfil,
            emp.descrfil,
            cab.nrcpm,
            cab.dtpagefe,
            cab.dteminfce,
            cab.cdcli,
            cli.nomecli,
            cab.cdfunre,
            ven.nomefun,
            it.itemid,
            it.cdpro,
            pr.descrprd,
            pr.setor,
            it.quant,
            it.pruni,
            it.vrtot,
            it.vrdsc,
            it.vrrcb,
            it.prcusto,
            it.prcompra
        FROM FC14000 as cab"
    );
    
    // Adicionar todos os joins
    query.push_str(" INNER JOIN (SELECT company_id, cnpj, companygroupname FROM company_config) cfg");
    query.push_str(" ON cab.company_id = cfg.company_id AND cab.companygroupname = cfg.companygroupname");
    query.push_str(" LEFT JOIN (SELECT company_id, cdcli, nomecli FROM fc07000) cli");
    query.push_str(" ON cab.company_id = cli.company_id AND cab.cdcli = cli.cdcli");
    query.push_str(" INNER JOIN (SELECT company_id, cdfun, nomefun FROM fc08000 GROUP BY company_id, cdfun, nomefun) ven");
    query.push_str(" ON cab.company_id = ven.company_id AND cab.cdfunre = ven.cdfun");
    query.push_str(" INNER JOIN (SELECT company_id, cdfil, nrcpm, itemid, cdpro, quant, CAST(pruni as numeric) pruni, CAST(vrtot as numeric) vrtot, CAST(vrdsc as numeric) vrdsc, ROUND(CAST((vrtot+vrtxav) - (vrdsc + vrdscv) as numeric),2) vrrcb, prcusto, prcompra FROM fc14100) it");
    query.push_str(" ON it.company_id = cab.company_id AND it.cdfil = cab.cdfil AND it.nrcpm = cab.nrcpm");
    query.push_str(" LEFT JOIN (SELECT company_id, cdpro, descrprd, setor FROM fc03000 pr WHERE 1=1) pr");
    query.push_str(" ON it.company_id = pr.company_id AND it.cdpro = pr.cdpro");
    query.push_str(" INNER JOIN (SELECT company_id, cdfil, descrfil FROM companies) emp");
    query.push_str(" ON cab.company_id = emp.company_id AND cab.cdfil = emp.cdfil");
    query.push_str(" INNER JOIN (SELECT company_id, company_name FROM company_config) cc");
    query.push_str(" ON cab.company_id = cc.company_id");
    query.push_str(" WHERE pr.cdpro IS NOT NULL");

    // Adicionar filtros para visualização
    if let Some(data_inicio) = &filtros.data_inicio {
        query.push_str(&format!(" AND cab.dtpagefe >= '{}'", data_inicio));
    }
    if let Some(data_fim) = &filtros.data_fim {
        query.push_str(&format!(" AND cab.dtpagefe <= '{}'", data_fim));
    }
    if let Some(empresa) = &filtros.empresa {
        query.push_str(&format!(" AND cab.companygroupname = '{}'", empresa));
    }
    if let Some(filial) = &filtros.filial {
        query.push_str(&format!(" AND cab.cdfil = {}", filial));
    }
    if let Some(vendedor) = &filtros.vendedor {
        query.push_str(&format!(" AND cab.cdfunre = {}", vendedor));
    }
    if let Some(produto) = &filtros.produto {
        query.push_str(&format!(" AND pr.descrprd ILIKE '%{}%'", produto));
    }

    query.push_str(" ORDER BY cab.dtpagefe, cab.company_id");
    
    if let Some(limite) = filtros.limite {
        query.push_str(&format!(" LIMIT {}", limite));
    }

    Ok(HttpResponse::Ok().json(json!({
        "query": query,
        "filtros_recebidos": {
            "data_inicio": filtros.data_inicio,
            "data_fim": filtros.data_fim,
            "empresa": filtros.empresa,
            "filial": filtros.filial,
            "vendedor": filtros.vendedor,
            "produto": filtros.produto,
            "limite": filtros.limite
        }
    })))
}
