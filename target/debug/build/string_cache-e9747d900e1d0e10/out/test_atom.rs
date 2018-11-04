pub type TestAtom = :: string_cache :: Atom < TestAtomStaticSet > ;
pub struct TestAtomStaticSet ;
impl :: string_cache :: StaticAtomSet for TestAtomStaticSet { fn get ( ) -> & 'static :: string_cache :: PhfStrSet { static SET : :: string_cache :: PhfStrSet = :: string_cache :: PhfStrSet { key : 6246114685207409605u64 , disps : & [ ( 0u32 , 4u32 ) , ( 2u32 , 0u32 ) , ( 0u32 , 8u32 ) ] , atoms : &[
"",
"head",
"body",
"area",
"address",
"font-weight",
"br",
"b",
"id",
"a",
"html" ] , hashes : & [ 2699410566u32 , 1535388292u32 , 3377026183u32 , 341178065u32 , 2737924581u32 , 3209338327u32 , 4104588231u32 , 2578680001u32 , 1018728650u32 , 826473443u32 , 512758224u32 ] } ;
& SET } fn empty_string_index ( ) -> u32 { 0u32 } } # [ macro_export ] macro_rules ! test_atom {
( "" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x2u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "head" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x100000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "body" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x200000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "area" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x300000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "address" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x400000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "font-weight" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x500000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "br" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x600000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "b" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x700000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "id" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x800000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "a" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0x900000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
( "html" ) => { $ crate :: atom :: tests :: TestAtom { unsafe_data : 0xA00000002u64 , phantom : :: std :: marker :: PhantomData , } } ;
}