use crate::telegram::model::action_id::ActionId;
use crate::telegram::model::action_route::ActionRoute;
use crate::telegram::model::outgoing_message_id::OutgoingMessageId;
use std::iter::once;

#[derive(new, Debug, Serialize, Deserialize)]
pub struct Page {
    #[serde(rename="number")]
    number: usize,
    #[serde(rename="route")]
    route: ActionRoute,
}

impl Page {
    pub fn get_number(&self) -> usize {
        return self.number;
    }

    pub fn get_route(&self) -> &ActionRoute {
        return &self.route;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagerAction {
    #[serde(rename="outgoing_message_id")]
    outgoing_message_id: OutgoingMessageId,
    #[serde(rename="action_id")]
    action_id: ActionId,
    #[serde(rename="current_page_num")]
    current_page_num: usize,
    #[serde(rename="page_size")]
    page_size: usize,
    #[serde(rename="pages")]
    pages: Vec<Page>,
    #[serde(rename="paginator_length")]
    paginator_length: usize,
}

impl PagerAction {
    const PAGE_SIZE: usize = 10;
    const PAGINATOR_LENGTH: usize = 2;

    pub fn new(action_id: ActionId, items_count: usize) -> PagerAction {
        let page_size = Self::PAGE_SIZE;
        let pages_count = Self::compute_pages_count(items_count, page_size);
        let pages = Self::create_pages(&action_id, pages_count);
        return PagerAction {
            outgoing_message_id: OutgoingMessageId::new(),
            action_id,
            current_page_num: 0,
            page_size,
            pages,
            paginator_length: Self::PAGINATOR_LENGTH,
        }
    }

    pub fn get_outgoing_message_id(&self) -> &OutgoingMessageId {
        return &self.outgoing_message_id;
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
                number: page,
                route: action_id.create_route(),
            };
            pages.push(page);
        }
        return pages;
    }

    pub fn iter_records<'a, T>(&'a self, iterator: impl Iterator<Item=T> + 'a) -> impl Iterator<Item=T> + 'a {
        let skip = self.current_page_num * self.page_size;
        return iterator.skip(skip).take(self.page_size);
    }

    pub fn is_selected(&self, page: &Page) -> bool {
        return self.current_page_num == page.number;
    }

    pub fn iter_pages(&self) -> impl Iterator<Item=&Page> + '_ {
        let pages_length = self.pages.len();
        let begin = self.current_page_num - self.paginator_length;
        let end = self.current_page_num + self.paginator_length;
        return self
            .pages
            .iter()
            .filter(move |&page| {
                if page.number == 0 {
                    return true;
                }
                if page.number == pages_length {
                    return true;
                }
                if page.number >= begin && page.number <= end {
                    return true;
                }
                return false;
            });
    }
}