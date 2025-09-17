use aidoku::*;

// TODO: Implement actual scraping logic with Copilot help
#[get_manga_list]
fn get_manga_list(_filters: Filters, _page: i32) -> Result<MangaPage> {
    unimplemented!()
}

#[get_manga_details]
fn get_manga_details(_id: String) -> Result<Manga> {
    unimplemented!()
}

#[get_chapter_list]
fn get_chapter_list(_id: String) -> Result<Vec<Chapter>> {
    unimplemented!()
}

#[get_page_list]
fn get_page_list(_id: String) -> Result<Vec<Page>> {
    unimplemented!()
}
