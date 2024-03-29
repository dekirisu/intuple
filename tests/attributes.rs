use intuple::*;

/* -------------------------------------------------------------------------- */
/*                                  Ignoring                                  */
/* -------------------------------------------------------------------------- */
/* --------------------------------- Unnamed -------------------------------- */

    #[derive(Intuple,Debug,PartialEq)]
    struct Unnamed(u32, #[igno] u32, u32);

    #[test]
    fn attr_igno_unnamed_from_tuple() {
        let a = Unnamed::from_tuple((2,1));
        assert_eq!(a, Unnamed(2,0,1));
    }

    #[test]
    fn attr_igno_unnamed_to_tuple() {
        let a = Unnamed(2,0,1).into_tuple();
        assert_eq!(a, (2,1));
    }

    #[test]
    fn attr_igno_unnamed_to_tuple_ref() {
        let a = Unnamed(2,0,1).as_tuple_ref();
        assert_eq!(a,(&2,&1));
    }    

    #[test]
    fn attr_igno_unnamed_to_tuple_ref_mut() {
        let mut a = Unnamed(2,0,1);
        let b = a.as_tuple_ref_mut();
        assert_eq!(b,(&mut 2,&mut 1));
    }

/* ---------------------------------- Named --------------------------------- */

    #[derive(Intuple,Debug,PartialEq)]
    struct Named {a:u32, #[igno] b:u32, c:u32}

    #[test]
    fn attr_igno_named_from_tuple() {
        let a = Named::from_tuple((2,1));
        assert_eq!(a, Named{a:2,b:0,c:1});
    }

    #[test]
    fn attr_igno_named_to_tuple() {
        let a = Named{a:2,b:0,c:1}.into_tuple();
        assert_eq!(a, (2,1));
    }

    #[test]
    fn attr_igno_named_to_tuple_ref() {
        let a = Named{a:2,b:0,c:1}.as_tuple_ref();
        assert_eq!(a,(&2,&1));
    }    

    #[test]
    fn attr_igno_named_to_tuple_ref_mut() {
        let mut a = Named{a:2,b:0,c:1};
        let b = a.as_tuple_ref_mut();
        assert_eq!(b,(&mut 2,&mut 1));
    }

/* -------------------------------------------------------------------------- */
/*                                  Recursion                                 */
/* -------------------------------------------------------------------------- */
/* --------------------------------- 1 Level -------------------------------- */

    #[derive(Intuple,Debug,PartialEq)]
    struct RecursiveA{a:u32, #[rcsv] b:Unnamed, c:Unnamed}

    #[test]
    fn attr_recursive_1level() {
        let mut rcsv = RecursiveA::from_tuple((10,(10,20),Unnamed(10,5,20)));  
        assert_eq!(rcsv, RecursiveA{a:10,b:Unnamed(10,0,20),c:Unnamed(10,5,20)});
        
        let rref = rcsv.as_tuple_ref(); 
        assert_eq!(rref, (&10,(&10,&20),&Unnamed(10,5,20)));
        let rref = rcsv.as_tuple_ref_mut(); 
        assert_eq!(rref, (&mut 10,(&mut 10,&mut 20),&mut Unnamed(10,5,20)));

        let rcsv = rcsv.into_tuple(); 
        assert_eq!(rcsv, (10,(10,20),Unnamed(10,5,20)));
    }

/* -------------------------------- 2 Levels -------------------------------- */

    #[derive(Intuple,Debug,PartialEq)]
    struct RecursiveB(u32, #[rcsv] RecursiveA, RecursiveA);

    #[test]
    fn attr_recursive_2levels() {
        let rcsv_a = RecursiveA::from_tuple((10,(10,20),Unnamed(10,5,20)));  
        let mut rcsv = RecursiveB::from_tuple((33,(11,(11,22),Unnamed(11,7,22)),rcsv_a));
        assert_eq!(rcsv, RecursiveB(
            33,
            RecursiveA{a:11,b:Unnamed(11,0,22),c:Unnamed(11,7,22)},
            RecursiveA{a:10,b:Unnamed(10,0,20),c:Unnamed(10,5,20)}
        ));

        let rref = rcsv.as_tuple_ref(); 
        assert_eq!(rref, (&33,(&11,(&11,&22),&Unnamed(11,7,22)),&RecursiveA{a:10,b:Unnamed(10,0,20),c:Unnamed(10,5,20)}));
        let rref = rcsv.as_tuple_ref_mut(); 
        assert_eq!(rref, (&mut 33,(&mut 11,(&mut 11,&mut 22),&mut Unnamed(11,7,22)),&mut RecursiveA{a:10,b:Unnamed(10,0,20),c:Unnamed(10,5,20)}));

        let rcsv = rcsv.into_tuple(); 
        assert_eq!(rcsv, (33,(11,(11,22),Unnamed(11,7,22)),RecursiveA{a:10,b:Unnamed(10,0,20),c:Unnamed(10,5,20)}));
    }

/* ----------------------------------- EOF ---------------------------------- */