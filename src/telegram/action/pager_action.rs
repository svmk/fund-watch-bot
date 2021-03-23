use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_route::ActionRoute;
use crate::prelude::*; 

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename="index")]
    index: usize,
    #[serde(rename="route")]
    route: ActionRoute,
}

impl Page {
    pub fn get_page(&self) -> usize {
        return self.index + 1;
    }
    
    fn get_index(&self) -> usize {
        return self.index;
    }

    pub fn get_route(&self) -> &ActionRoute {
        return &self.route;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Paginator {
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="current_page_index")]
    current_page_index: usize,
    #[serde(rename="page_size")]
    page_size: usize,
    #[serde(rename="pages")]
    pages: Vec<Page>,
    #[serde(rename="paginator_length")]
    paginator_length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagerAction {
    #[serde(rename="paginator")]
    paginator: Paginator,
}

impl PagerAction {
    const PAGE_SIZE: usize = 10;
    const PAGINATOR_LENGTH: usize = 2;

    pub fn new(action_id: ActionId, items_count: usize) -> PagerAction {
        let page_size = Self::PAGE_SIZE;
        let pages_count = Self::compute_pages_count(items_count, page_size);
        let pages = Self::create_pages(&action_id, pages_count);
        return PagerAction {
            paginator: Paginator {
                action_id,
                current_page_index: 0,
                page_size,
                pages,
                paginator_length: Self::PAGINATOR_LENGTH,
            },
        }
    }

    fn compute_pages_count(items_count: usize, page_size: usize) -> usize {
        let mut result = items_count / page_size;
        let items_count_overflow = items_count % page_size;
        if items_count_overflow != 0 {
            result = result + 1;
        }
        return result;
    }

    fn create_pages(action_id: &ActionId, pages_count: usize) -> Vec<Page> {
        let mut pages = Vec::new();
        for page in 0..pages_count {
            let page = Page {
                index: page,
                route: action_id.create_route(),
            };
            pages.push(page);
        }
        return pages;
    }

    pub fn iter_items<'a, T>(&'a self, iterator: impl Iterator<Item=T> + 'a) -> impl Iterator<Item=T> + 'a {
        let skip = self.paginator.current_page_index * self.paginator.page_size;
        let iterator = iterator
            .skip(skip)
            .take(self.paginator.page_size);
        return iterator;
    }

    pub fn iter_pages(&self) -> impl Iterator<Item=&Page> + '_ {
        let pages_length = self.paginator.pages.len();
        let begin = self.paginator.current_page_index.saturating_sub(self.paginator.paginator_length);
        let end = self.paginator.current_page_index + self.paginator.paginator_length;
        return self
            .paginator
            .pages
            .iter()
            .filter(move |&page| {
                if page.index == 0 {
                    return true;
                }
                if page.index == pages_length {
                    return true;
                }
                if page.index >= begin && page.index <= end {
                    return true;
                }
                return false;
            });
    }

    pub fn get_page_by_route(&self, route: &ActionRoute) -> Option<&Page> {
        return self.paginator.pages.iter().find(|page| {
            return page.get_route() == route;
        });
    }

    pub fn select_page(&mut self, page: &Page) -> Result<(), Failure> {
        let page = page.get_index();
        self.paginator.current_page_index = page;
        if page >= self.paginator.pages.len() {
            return crate::fail!("Unknown page {}", page);
        }
        return Ok(());
    }
}