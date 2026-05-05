//! 查询解析器模块
//! 
//! 解析和执行搜索查询。

use std::collections::HashMap;

/// 查询类型
#[derive(Debug, Clone)]
pub enum QueryType {
    /// 关键词搜索
    Keyword(String),
    /// 布尔搜索
    Boolean(BooleanQuery),
    /// 范围搜索
    Range(RangeQuery),
    /// 元数据搜索
    Metadata(String, String),
}

/// 布尔查询
#[derive(Debug, Clone)]
pub struct BooleanQuery {
    /// 操作符
    pub operator: BooleanOperator,
    
    /// 左操作数
    pub left: Box<QueryType>,
    
    /// 右操作数
    pub right: Box<QueryType>,
}

/// 布尔操作符
#[derive(Debug, Clone)]
pub enum BooleanOperator {
    /// AND
    And,
    /// OR
    Or,
    /// NOT
    Not,
}

/// 范围查询
#[derive(Debug, Clone)]
pub struct RangeQuery {
    /// 字段名
    pub field: String,
    
    /// 最小值
    pub min: Option<String>,
    
    /// 最大值
    pub max: Option<String>,
}

/// 解析后的查询
#[derive(Debug, Clone)]
pub struct ParsedQuery {
    /// 查询类型
    pub query_type: QueryType,
    
    /// 最大结果数
    pub limit: usize,
    
    /// 偏移量
    pub offset: usize,
    
    /// 排序字段
    pub sort_by: Option<String>,
    
    /// 是否升序
    pub ascending: bool,
}

/// 查询解析器
pub struct QueryParser;

impl QueryParser {
    /// 解析查询字符串
    pub fn parse(query: &str) -> Result<ParsedQuery, String> {
        // 简化版本：解析基本查询
        let parts: Vec<&str> = query.splitn(2, ':').collect();
        
        if parts.len() == 2 {
            // 元数据查询: key:value
            Ok(ParsedQuery {
                query_type: QueryType::Metadata(
                    parts[0].to_string(),
                    parts[1].to_string(),
                ),
                limit: 10,
                offset: 0,
                sort_by: None,
                ascending: true,
            })
        } else {
            // 关键词查询
            Ok(ParsedQuery {
                query_type: QueryType::Keyword(query.to_string()),
                limit: 10,
                offset: 0,
                sort_by: None,
                ascending: true,
            })
        }
    }
    
    /// 解析带参数的查询
    pub fn parse_with_params(query: &str, params: &HashMap<String, String>) -> Result<ParsedQuery, String> {
        let mut parsed = Self::parse(query)?;
        
        if let Some(limit) = params.get("limit") {
            parsed.limit = limit.parse().unwrap_or(10);
        }
        
        if let Some(offset) = params.get("offset") {
            parsed.offset = offset.parse().unwrap_or(0);
        }
        
        if let Some(sort) = params.get("sort") {
            parsed.sort_by = Some(sort.clone());
        }
        
        if let Some(order) = params.get("order") {
            parsed.ascending = order == "asc";
        }
        
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword_query() {
        let parsed = QueryParser::parse("hello world").unwrap();
        match parsed.query_type {
            QueryType::Keyword(q) => assert_eq!(q, "hello world"),
            _ => panic!("Expected Keyword query"),
        }
    }

    #[test]
    fn test_parse_metadata_query() {
        let parsed = QueryParser::parse("type:credential").unwrap();
        match parsed.query_type {
            QueryType::Metadata(key, value) => {
                assert_eq!(key, "type");
                assert_eq!(value, "credential");
            }
            _ => panic!("Expected Metadata query"),
        }
    }

    #[test]
    fn test_parse_with_params() {
        let mut params = HashMap::new();
        params.insert("limit".to_string(), "5".to_string());
        params.insert("sort".to_string(), "name".to_string());
        
        let parsed = QueryParser::parse_with_params("hello", &params).unwrap();
        assert_eq!(parsed.limit, 5);
        assert_eq!(parsed.sort_by, Some("name".to_string()));
    }
}
