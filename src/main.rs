mod typesafe;

use std::error::Error;
use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

struct Datum<'d>(PhantomData<&'d ()>);

#[derive(Debug)]
struct SpiError;

trait IntoDatum {}

trait FromDatum {
    unsafe fn from<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd;
    unsafe fn from_in_mem_ctx<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd;
}

impl FromDatum for i32 {
    unsafe fn from<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd {
        todo!()
    }

    unsafe fn from_in_mem_ctx<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd {
        todo!()
    }
}

impl<'a> FromDatum for &'a str {
    unsafe fn from<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd {
        todo!()
    }

    unsafe fn from_in_mem_ctx<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd {
        todo!()
    }
}

impl FromDatum for String {
    unsafe fn from<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd {
        todo!()
    }

    unsafe fn from_in_mem_ctx<'d>(datum: Datum<'d>) -> Option<Self> where Self: Sized + 'd {
        todo!()
    }
}

struct Spi;

struct SpiClient<'c>(PhantomData<&'c ()>);

struct Table<'t>(PhantomData<&'t ()>);

struct Row<'r>(PhantomData<&'r ()>);

impl Spi {

    fn get_one<T: FromDatum>(q: &str) -> Result<Option<T>, SpiError> {
        Spi::connect(|client| client.select(q).next_row().unwrap().get(1))
    }

    unsafe fn get_one_unchecked<T: FromDatum>(q: &str) -> Option<T> {
        Spi::connect(|client| client.select(q).next_row().unwrap().get_unchecked(1))
    }

    fn connect_better<R: FromDatum + IntoDatum, F: Fn(&SpiClient) -> R>(f: F) -> R {
        R::from_in_mem_ctx(f(todo!()).into_datum(), ctx)
        // TODO: how costly is into_datum for str
        // TOD: can we instead have a trait to "copy" From/Into Datum in a different ctx
        // TODO: maybe a
    }
    fn connect<R, F: Fn(&SpiClient) -> R>(f: F) -> R {
        // TODO: save memory context
        let client = SpiClient(PhantomData); // TODO memctx

        let r  = f(&client);

        // let r = R::from_datum_in_memory_ctx(datum, saved_ctx)

        // close spi mem context
        r
    }

}

impl<'c> SpiClient<'c> {
    fn select(&'c self, q: &str) -> Table<'c> {
        todo!()
    }
}

impl<'t> Table<'t> {
    fn next_row(&'t mut self) -> Option<Row<'t>> {
        todo!()
    }

}
impl<'r> Row<'r> {

    fn get<T: FromDatum>(&'r self, i: usize) -> Result<Option<T>, SpiError>  {
        unsafe { Ok(self.get_unchecked(i)) }
    }

    unsafe fn get_unchecked<T: FromDatum + 'r>(&'r self, i: usize) -> Option<T> {
        let datum: Datum<'r> = todo!();
        /// SAFETY: caveat emptor
        unsafe { T::from(datum) }
    }
}

fn foobar() {

    // Type correctness
    Spi::get_one::<String>("select 1");

    Spi::connect(|client| {
        let x: String = client.select("select 1").next_row().unwrap().get(1)
            .expect("Not the correct type")
            .expect("Not Some");

        let y: String = unsafe  { client.select("select 1").next_row().unwrap().get_unchecked(1) }.expect("Not Some");



    });

    // Lifetimes
    let res: &str = Spi::connect(|client| {
        let x = client.select("select 'mario'").next_row().unwrap();
        x.get(1).unwrap().unwrap()
        // .get(1)
        //     .expect("Not the correct type")
        //     .expect("Not Some");
        // x
    });


    let x = Spi::connect(|client| client);

    // TODO: can we build a ShortDatum that borrows client and wraps a datum?
    // TODO:
}