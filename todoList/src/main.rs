
struct Book{
    author: String,
    title: String,
    is_checked_out: bool
}

struct Library{
    shelves: Vec<Book>
}

impl Library{
    fn new() -> Library{Library{shelves: Vec::new(),}}

    fn add(&mut self, author: String, title: String){
        self.shelves.push(Book {author, title, is_checked_out: false});
    }

    fn add(&mut self, book: Book){
        self.shelves.push(book);
    }

}
