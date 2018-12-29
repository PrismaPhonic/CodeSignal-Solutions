#![feature(test)]
use std::thread;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

extern crate test;

/** 
 * The problem basically specifies that we return all combinations that
 * result from removing exactly one city from the graph.  We 
 * just need to iterate through and for a given city # remove that 
 * column and row and then return that pushed to a larger array
 * 
 * First Solution:
 * This solution is a bit naive.  We are looping through the input matrix n times
 * where n is the number of cities.  This happens when you think from a deletion perspective.
 * If we instead consider looping through the input matrix only once and adding to appropriate possibility matrices we can land a more efficient solution
 * 
 * Benchmark results:
 * test tests::bench_financial_crisis       ... bench:       3,829 ns/iter (+/- 192)
 * test tests::bench_financial_crisis_naive ... bench:       4,585 ns/iter (+/- 292)
 * )
 */

// fn financialCrisisNaive(roadRegister: Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
//     let city_count = roadRegister.len();
//     let rr = Arc::new(roadRegister);
//     let mut possibilities: Vec<Vec<Vec<bool>>> = Vec::new();
//     let mut children = Vec::new();
//     for i in 0..city_count {
//         let r1 = Arc::clone(&rr);
//         children.push(thread::spawn(move || -> Vec<Vec<bool>> {
//             r1.iter().enumerate()
//                 .filter(|(index, _row)| index != &i)
//                 .map(|(_index, row)| {
//                     row.into_iter().enumerate()
//                         .filter(|(idx, _city)| idx != &i)
//                         .map(|(_i, city)| *city)
//                         .collect()
//                 })
//                 .collect()
//         }));
//     };

//     for child in children { 
//         possibilities.push(child.join().unwrap());
//     }

//     possibilities
// }

fn financialCrisisNaive(roadRegister: Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let city_count = roadRegister.len();
    let mut possibilities: Vec<Vec<Vec<bool>>> = vec![Vec::new(); city_count];

    possibilities.iter_mut().enumerate().for_each(|(i, possibility)| {
        *possibility = roadRegister.iter().enumerate()
            .filter(|(index, _row)| index != &i)
            .map(|(_index, row)| {
                row.into_iter().enumerate()
                    .filter(|(idx, _city)| idx != &i)
                    .map(|(_i, city)| *city)
                    .collect()
            })
            .collect();
    });

    possibilities
}


/**
 * This solution is better from a performance standpoint but certainly
 * more verbose and not as readable.
 */

pub extern fn financialCrisis(roadRegister: Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let city_count = roadRegister.len();
    let mut possibilities: Vec<Vec<Vec<bool>>> = Vec::new();

    for _c in 0..city_count {
        possibilities.push(Vec::new());
    }

    for i in 0..city_count {
        
        // let's start each round of city elimation by 
        // pushing a new array to each possibilities that
        // are NOT the city being eliminated
        for l in 0..city_count {
            if i != l {
                possibilities[l].push(Vec::new());
            }
        }

        // array to iterate through input matrix rows
        for j in 0..city_count {
            
            // k moves through our possibilities of road registers and
            // push boolean from input to appropriate possibility matrices
            for k in 0..city_count {
                let latest = possibilities[k].len() - 1;
                    if i != k && j != k {
                        possibilities[k][latest].push(roadRegister[i][j]);
                    }
            }
        }
    }
    possibilities
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn test_financial_crisis_naive() {
        let mut testRegister: Vec<Vec<bool>> = vec![
            vec![false,false,false,false,true,false], 
            vec![false,false,true,false,true,false], 
            vec![false,true,false,true,true,true], 
            vec![false,false,true,false,false,false], 
            vec![true,true,true,false,false,true], 
            vec![false,false,true,false,true,false]
        ];

        assert_eq!(
                   financialCrisisNaive(testRegister.clone()),
                   [[[false,true,false,true,false],[true,false,true,true,true],[false,true,false,false,false],[true,true,false,false,true],[false,true,false,true,false]], 
 [[false,false,false,true,false],[false,false,true,true,true],[false,true,false,false,false],[true,true,false,false,true],[false,true,false,true,false]], 
 [[false,false,false,true,false],[false,false,false,true,false],[false,false,false,false,false],[true,true,false,false,true],[false,false,false,true,false]], 
 [[false,false,false,true,false],[false,false,true,true,false],[false,true,false,true,true],[true,true,true,false,true],[false,false,true,true,false]], 
 [[false,false,false,false,false],[false,false,true,false,false],[false,true,false,true,true],[false,false,true,false,false],[false,false,true,false,false]], 
 [[false,false,false,false,true],[false,false,true,false,true],[false,true,false,true,true],[false,false,true,false,false],[true,true,true,false,false]]] 
        );
    }

    #[bench]
    fn bench_financial_crisis_naive(b: &mut Bencher) {
        let mut testRegister: Vec<Vec<bool>> = vec![
            vec![false,false,false,false,true,false], 
            vec![false,false,true,false,true,false], 
            vec![false,true,false,true,true,true], 
            vec![false,false,true,false,false,false], 
            vec![true,true,true,false,false,true], 
            vec![false,false,true,false,true,false]
        ];

        b.iter(|| {
            financialCrisisNaive(testRegister.clone())
        });
    }

    #[bench]
    fn bench_financial_crisis(b: &mut Bencher) {
        let mut testRegister: Vec<Vec<bool>> = vec![
            vec![false,false,false,false,true,false], 
            vec![false,false,true,false,true,false], 
            vec![false,true,false,true,true,true], 
            vec![false,false,true,false,false,false], 
            vec![true,true,true,false,false,true], 
            vec![false,false,true,false,true,false]
        ];

        b.iter(|| {
            financialCrisis(testRegister.clone())
        });
    }
}
