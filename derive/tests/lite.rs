use intuple_derive::*;

/* --------------------------------- Unnamed -------------------------------- */

    #[derive(IntupleLite,Debug,PartialEq)]
    struct Unnamed<T>(u32, u32, T);

    #[test]
    fn lite_unnamed_from_tuple() {
        let a = Unnamed::<u16>::from((3,2,1));
        assert_eq!(a, Unnamed::<u16>(3,2,1));
    }

    #[test]
    fn lite_tuple_into_unnamed() {
        let a: Unnamed::<u16> = (3,2,1).into();
        assert_eq!(a, Unnamed::<u16>(3,2,1));
    }

    #[test]
    fn lite_tuple_from_unnamed() {
        let a = <(u32, u32, u16)>::from(Unnamed::<u16>(3,2,1));
        assert_eq!(a, (3,2,1));
    }

    #[test]
    fn lite_unnamed_into_tuple() {
        let a: (u32, u32, u16) = Unnamed::<u16>(3,2,1).into();
        assert_eq!(a, (3,2,1));
    }

/* ---------------------------------- Named --------------------------------- */

    #[derive(IntupleLite,Debug,PartialEq)]
    struct Named<T> {a:u32, b:u32, c:T}

    #[test]
    fn lite_named_from_tuple() {
        let a = Named::<u16>::from((3,2,1));
        assert_eq!(a, Named::<u16>{a:3,b:2,c:1});
    }

    #[test]
    fn lite_tuple_into_named() {
        let a: Named::<u16> = (3,2,1).into();
        assert_eq!(a, Named::<u16>{a:3,b:2,c:1});
    }

    #[test]
    fn lite_tuple_from_named() {
        let a = <(u32, u32, u16)>::from(Named::<u16>{a:3,b:2,c:1});
        assert_eq!(a, (3,2,1));
    }

    #[test]
    fn lite_named_into_tuple() {
        let a: (u32, u32, u16) = Named::<u16>{a:3,b:2,c:1}.into();
        assert_eq!(a, (3,2,1));
    }

/* ----------------------------------- EOF ---------------------------------- */