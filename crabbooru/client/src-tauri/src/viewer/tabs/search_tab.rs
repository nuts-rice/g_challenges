use crate::model::SearchQuery;

#[derive(Debug, Default, Clone, PartialEq, )]
pub struct SearchTab {
    pub query: SearchQuery,
    pub results: Vec<SearchQuery>,
    pub selected: usize,
}


impl SearchTab