use prusti_contracts::*;
use crate::option_spec::*;
use crate::result_spec::*;

pub struct EnableFilter(usize);

impl EnableFilter {
    #[pure]
    pub fn value(&self) ->  usize {
        self.0
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FilterProtocol {
    Tcp = 0,
    Udp = 1,
    Sctp = 2,
    Other = 3
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum L5FilterPriority {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum QueueID {
    Q0,Q1,Q2,Q3,Q4,Q5,Q6,Q7,Q8,Q9,
    Q10,Q11,Q12,Q13,Q14,Q15,Q16,Q17,Q18,Q19,
    Q20,Q21,Q22,Q23,Q24,Q25,Q26,Q27,Q28,Q29,
    Q30,Q31,Q32,Q33,Q34,Q35,Q36,Q37,Q38,Q39,
    Q40,Q41,Q42,Q43,Q44,Q45,Q46,Q47,Q48,Q49,
    Q50,Q51,Q52,Q53,Q54,Q55,Q56,Q57,Q58,Q59,
    Q60,Q61,Q62,Q63,Q64
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FilterParameters {
    source_ip: [u8; 4],
    dest_ip: [u8; 4],
    source_port: u16,
    dest_port: u16,
    protocol: FilterProtocol,
    priority: L5FilterPriority,
    qid: QueueID
}

impl FilterParameters {
    #[pure]
    fn parameters_equal(&self, other: &Self) -> bool {
        self.source_ip == other.source_ip &&
        self.dest_ip == other.dest_ip &&
        self.source_port == other.source_port &&
        self.dest_port == other.dest_port &&
        self.protocol == other.protocol &&
        self.priority == other.priority
    }
}

// #[ensures(result.is_ok() ==> {
//     let idx = peek_result_ref(&result).value(); 
//     filters[idx].is_some() 
//     && 
//     peek_option(&filters[idx]) == new_filter
// })]
// #[ensures(result.is_err() ==> {
//     match peek_err(&result) {
//         FilterError::NoneAvailable => forall(|i: usize|( 0 <= i && i < 128 ==> filters[i] == old(filters[i]))),
//         FilterError::IdenticalFilter(idx) => filters[idx].is_some() && peek_option(&filters[idx]).parameters_equal(&new_filter),
//         _ => true
//     }
// })]
// pub fn add_filter(filters: &mut [Option<FilterParameters>; 128], new_filter: FilterParameters) -> Result<usize, FilterError> {
//     let mut i = 0;
//     while i < 128 {
//         body_invariant!(0 <= i && i < 128);
//         if filters[i].is_some() {
//             let filter = filters[i].unwrap();
//             if filter.parameters_equal(&new_filter) {
//                 return Err(FilterError::IdenticalFilter(i));
//             }
//         } else {
//             filters[i] = Some(new_filter);
//             return Ok(i);
//         }
//         i += 1;
//     }
//     return Err(FilterError::NoneAvailable)
// }

#[ensures(result.is_ok() ==> {
    forall(|i: usize| 0 <= i && i < 128 ==> {
        if i == peek_result_ref(&result).value() {
            filters[i].is_some() && peek_option(&filters[i]) == new_filter
        } else {
            filters[i] == old(filters[i]) && (filters[i].is_some() ==> !peek_option(&filters[i]).parameters_equal(&new_filter))
        }
    } )
})]
#[ensures(result.is_err() ==> {
    match peek_err(&result) {
        FilterError::NoneAvailable => forall(|i: usize|( 0 <= i && i < 128 ==> filters[i].is_some())),
        FilterError::IdenticalFilter(idx) => filters[idx].is_some() && peek_option(&filters[idx]).parameters_equal(&new_filter),
    } && forall(|i: usize|( 0 <= i && i < 128 ==> filters[i] == old(filters[i])))
})]
pub fn check_and_add_filter(filters: &mut [Option<FilterParameters>; 128], new_filter: FilterParameters) -> Result<EnableFilter, FilterError> {
    let mut i = 0;
    let mut unused_filter = None ;

    while i < 128 {
        body_invariant!(0 <= i && i < 128);
        body_invariant!(unused_filter.is_some() ==> peek_option(&unused_filter) < filters.len());
        body_invariant!(forall( |x: usize| 0 <= x && x < i ==> {filters[x].is_some() ==> !peek_option(&filters[x]).parameters_equal(&new_filter)}));
        body_invariant!(unused_filter.is_none() ==> forall( |x: usize| 0 <= x && x < i ==> filters[x].is_some()));

        if filters[i].is_some() {
            if filters[i].unwrap().parameters_equal(&new_filter) {
                return Err(FilterError::IdenticalFilter(i));
            }
        } else if unused_filter.is_none(){
            unused_filter = Some(i);
        }
        i += 1;
    }
    if unused_filter.is_some() {
        let filter_idx = unused_filter.unwrap();
        filters[filter_idx] = Some(new_filter);
        Ok(EnableFilter(filter_idx))
    } else {
        Err(FilterError::NoneAvailable)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FilterError {
    NoneAvailable,
    IdenticalFilter(usize)
}