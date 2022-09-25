use intuple::*;

/* -------------------------------------------------------------------------- */
/*                                   Unnamed                                  */
/* -------------------------------------------------------------------------- */

    #[derive(Intuple,Debug,PartialEq)]
    struct Unnamed<T>(u32, u32, T);

    /* ----------------------------------- Std ---------------------------------- */

        #[test]
        fn var_std_unnamed_from_tuple() {
            let a = Unnamed::<u16>::from((3,2,1));
            assert_eq!(a, Unnamed(3,2,1));
        }

        #[test]
        fn var_std_tuple_into_unnamed() {
            let a: Unnamed::<u16> = (3,2,1).into();
            assert_eq!(a, Unnamed::<u16>(3,2,1));
        }

        #[test]
        fn var_std_tuple_from_unnamed() {
            let a = <(u32, u32, u16)>::from(Unnamed::<u16>(3,2,1));
            assert_eq!(a, (3,2,1));
        }

        #[test]
        fn var_std_unnamed_into_tuple() {
            let a: (u32, u32, u16) = Unnamed::<u16>(3,2,1).into();
            assert_eq!(a, (3,2,1));
        }

    /* --------------------------------- Intuple -------------------------------- */

        #[test]
        fn var_itp_unnamed_from_tuple() {
            let a = Unnamed::<u16>::from_tuple((3,2,1));
            assert_eq!(a, Unnamed::<u16>(3,2,1));
            let a = Unnamed::<u16>::fruple((3,2,1));
            assert_eq!(a, Unnamed::<u16>(3,2,1));
        }

        #[test]
        fn var_itp_tuple_into_unnamed() {
            let a: Unnamed::<u16> = (3,2,1).into_struct();
            assert_eq!(a, Unnamed::<u16>(3,2,1));
            let a: Unnamed::<u16> = (3,2,1).intruct();
            assert_eq!(a, Unnamed::<u16>(3,2,1));
        }

        #[test]
        fn var_itp_tuple_from_unnamed() {
            let a = <(u32, u32, u16)>::from_struct(Unnamed::<u16>(3,2,1));
            assert_eq!(a, (3,2,1));
            let a = <(u32, u32, u16)>::fruct(Unnamed::<u16>(3,2,1));
            assert_eq!(a, (3,2,1));
        }

        #[test]
        fn var_itp_unnamed_into_tuple() {
            let a: (u32, u32, u16) = Unnamed::<u16>(3,2,1).into_tuple();
            assert_eq!(a, (3,2,1));
            let a: (u32, u32, u16) = Unnamed::<u16>(3,2,1).intuple();
            assert_eq!(a, (3,2,1));
        }

/* -------------------------------------------------------------------------- */
/*                                    Named                                   */
/* -------------------------------------------------------------------------- */

    #[derive(Intuple,Debug,PartialEq)]
    struct Named<T>{a:u32, b:u32, c:T}

    /* ----------------------------------- Std ---------------------------------- */

        #[test]
        fn var_std_named_from_tuple() {
            let a = Named::<u16>::from((3,2,1));
            assert_eq!(a, Named::<u16>{a:3,b:2,c:1});
        }

        #[test]
        fn var_std_tuple_into_named() {
            let a: Named::<u16> = (3,2,1).into();
            assert_eq!(a, Named::<u16>{a:3,b:2,c:1});
        }

        #[test]
        fn var_std_tuple_from_named() {
            let a = <(u32, u32, u16)>::from(Named::<u16>{a:3,b:2,c:1});
            assert_eq!(a, (3,2,1));
        }

        #[test]
        fn var_std_named_into_tuple() {
            let a: (u32, u32, u16) = Named::<u16>{a:3,b:2,c:1}.into();
            assert_eq!(a, (3,2,1));
        }

    /* --------------------------------- Intuple -------------------------------- */

        #[test]
        fn var_itp_named_from_tuple() {
            let a = Named::<u16>::from_tuple((3,2,1));
            assert_eq!(a, Named{a:3,b:2,c:1});
            let a = Named::<u16>::fruple((3,2,1));
            assert_eq!(a, Named{a:3,b:2,c:1});
        }

        #[test]
        fn var_itp_tuple_into_named() {
            let a: Named::<u16> = (3,2,1).into_struct();
            assert_eq!(a, Named::<u16>{a:3,b:2,c:1});
            let a: Named::<u16> = (3,2,1).intruct();
            assert_eq!(a, Named::<u16>{a:3,b:2,c:1});
        }

        #[test]
        fn var_itp_tuple_from_named() {
            let a = <(u32, u32, u16)>::from_struct(Named::<u16>{a:3,b:2,c:1});
            assert_eq!(a, (3,2,1));
            let a = <(u32, u32, u16)>::fruct(Named::<u16>{a:3,b:2,c:1});
            assert_eq!(a, (3,2,1));
        }

        #[test]
        fn var_itp_named_into_tuple() {
            let a: (u32, u32, u16) = Named::<u16>{a:3,b:2,c:1}.into_tuple();
            assert_eq!(a, (3,2,1));
            let a: (u32, u32, u16) = Named::<u16>{a:3,b:2,c:1}.intuple();
            assert_eq!(a, (3,2,1));
        }

/* ----------------------------------- EOF ---------------------------------- */