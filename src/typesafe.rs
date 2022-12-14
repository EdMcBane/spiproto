use std::error::Error;

fn main() {
    println!("Hello, world!");
}

struct Datum;
#[derive(Debug)]
struct SpiError;

trait FromDatum {
    unsafe fn from(datum: Datum) -> Option<Self> where Self: Sized;
    unsafe fn from_in_mem_ctx(datum: Datum) -> Option<Self> where Self: Sized;
}

impl FromDatum for i32 {
    unsafe fn from(datum: Datum) -> Option<Self> {
        todo!()
    }

    unsafe fn from_in_mem_ctx(datum: Datum) -> Option<Self> {
        todo!()
    }
}

impl<'a> FromDatum for &'a str {
    unsafe fn from(datum: Datum) -> Option<Self> {
        todo!()
    }

    unsafe fn from_in_mem_ctx(datum: Datum) -> Option<Self> {
        todo!()
    }
}

impl FromDatum for String {
    unsafe fn from(datum: Datum) -> Option<Self> {
        todo!()
    }

    unsafe fn from_in_mem_ctx(datum: Datum) -> Option<Self> {
        todo!()
    }
}

struct Spi;

struct SpiClient;

struct Table;

struct Row;

impl Spi {

    fn get_one<T: FromDatum>(q: &str) -> Result<Option<T>, SpiError> {
        Spi::connect(|client| client.select(q).next_row().unwrap().get(1))
    }

    unsafe fn get_one_unchecked<T: FromDatum>(q: &str) -> Option<T> {
        Spi::connect(|client| client.select(q).next_row().unwrap().get_unchecked(1))
    }

    fn connect<R, F: Fn(SpiClient) -> R>(f: F) -> R {
        todo!()
    }

}

impl SpiClient {
    fn select(&self, q: &str) -> Table {
        todo!()
    }
}

impl Table {
    fn next_row(&mut self) -> Option<Row> {
        todo!()
    }

}
impl Row {

    fn get<T: FromDatum>(self, i: usize) -> Result<Option<T>, SpiError> {
        todo!()
    }

    unsafe fn get_unchecked<T: FromDatum>(self, i: usize) -> Option<T> {
        let datum: Datum = todo!();
        /// SAFETY: caveat emptor
        unsafe { T::from(datum) }
    }
}

fn foobar() {
    Spi::get_one::<String>("select 1");

    Spi::connect(|client| {
        let x: String = client.select("select 1").next_row().unwrap().get(1)
            .expect("Not the correct type")
            .expect("Not Some");

        let y: String = unsafe  { client.select("select 1").next_row().unwrap().get_unchecked(1) }.expect("Not Some");
    });
}