#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::collections::{};
use std::collections::VecDeque;
use std::collections::HashSet;
struct S1;
impl S1 {
    fn ladder_length(begin_word: String, end_word: String,
        word_list: Vec<String>)
        ->
            i32 {
            let mut word_list = word_list;
            word_list.insert(0, begin_word);

            let mut adjm =





                ::alloc::vec::from_elem(::alloc::vec::from_elem(false,
                        word_list.len()), word_list.len());
            let mut target = 0;
            {
                    let _t =
                        match #[lang = "into_iter"](#[lang = "Range"]{
                                            start: 0,

                                            end: adjm.len(),}) {
                                mut iter =>
                                    loop {
                                            match #[lang = "next"](&mut iter) {
                                                    #[lang = "None"] {} => break,
                                                    #[lang = "Some"] {  0: i } => {
                                                        {
                                                                let _t =
                                                                    match #[lang = "into_iter"](#[lang = "Range"]{
                                                                                        start: 0,

                                                                                        end: adjm.len(),}) {
                                                                            mut iter =>
                                                                                loop {
                                                                                        match #[lang = "next"](&mut iter) {
                                                                                                #[lang = "None"] {} => break,
                                                                                                #[lang = "Some"] {  0: j } => {
                                                                                                    adjm[i][j] = Self::is_adja(&word_list[i], &word_list[j]);
                                                                                                }
                                                                                            }
                                                                                    },
                                                                        };
                                                                _t
                                                            };
                                                        if target == 0 && word_list[i] == end_word { target = i; } }
                                                        }
                                                }, }; _t };
                    let mut q = VecDeque::new();
                    let mut visited =
                        ::alloc::vec::from_elem(false, word_list.len());
                    let mut dist = 1;
                    q.push_back(0);
                    visited[0] = true;
                    loop {
                            if !q.is_empty()
                                    {
                                            dist += 1;
                                            {
                                                    let _t =
                                                        match #[lang = "into_iter"](#[lang = "Range"]{
                                                                            start: 0,

                                                                            end: q.len(),}) {
                                                                mut iter =>
                                                                    loop {
                                                                            match #[lang = "next"](&mut iter) {
                                                                                    #[lang = "None"] {} => break,
                                                                                    #[lang = "Some"] {  0: _i } => {
                                                                                        let p = q.pop_front().unwrap();
                                                                                        {
                                                                                                let _t =
                                                                                                    match #[lang = "into_iter"](adjm[p].iter().enumerate()) {
                                                                                                            mut iter =>
                                                                                                                loop {
                                                                                                                        match #[lang = "next"](&mut iter) {
                                                                                                                                #[lang = "None"] {} => break,
                                                                                                                                #[lang = "Some"] {  0: (i, v) } => {
                                                                                                                                    if *v && !visited[i]
                                                                                                                                            {
                                                                                                                                                    q.push_back(i);
                                                                                                                                                    visited[i] = true;
                                                                                                                                                    if i == target { return dist; } } } } }, };
                                                                                                                _t
                                                                                                            } } } }, };
                                                                    _t
                                                                } } else { break; } } 0 }
                                    fn is_adja<'_, '_>(left: &'_ str, right: &'_ str)
                                        ->
                                            bool {
                                            left.chars().zip(right.chars()).map(|(l, r)|
                                                            { (l != r) as isize }).sum::<isize>() == 1
                                        }
                                }
                                struct S2;
                                impl S2 {
                                    fn ladder_length(begin_word: String, end_word: String,
                                        word_list: Vec<String>)
                                        ->
                                            i32 {
                                            let mut words: HashSet<String> =
                                                word_list.into_iter().collect();
                                            if !words.contains(&end_word) { return 0; }
                                                    let mut q = VecDeque::new();
                                                    let mut dist = 1;
                                                    words.remove(&begin_word);
                                                    q.push_back(begin_word);
                                                    loop {
                                                            if !q.is_empty()
                                                                    {
                                                                            dist += 1;
                                                                            {
                                                                                    let _t =
                                                                                        match #[lang = "into_iter"](#[lang = "Range"]{
                                                                                                            start: 0,

                                                                                                            end: q.len(),}) {
                                                                                                mut iter =>
                                                                                                    loop {
                                                                                                            match #[lang = "next"](&mut iter) {
                                                                                                                    #[lang = "None"] {} => break,
                                                                                                                    #[lang = "Some"] {  0: _ } => {
                                                                                                                        let ow = q.pop_front().unwrap();
                                                                                                                        {
                                                                                                                                let _t =
                                                                                                                                    match #[lang = "into_iter"](#[lang = "Range"]{
                                                                                                                                                        start: 0,

                                                                                                                                                        end: ow.len(),}) {
                                                                                                                                            mut iter =>
                                                                                                                                                loop {
                                                                                                                                                        match #[lang = "next"](&mut iter) {
                                                                                                                                                                #[lang = "None"] {} => break,
                                                                                                                                                                #[lang = "Some"] {  0: i } => {
                                                                                                                                                                    let mut o = ow.clone();
                                                                                                                                                                    {
                                                                                                                                                                            let _t =
                                                                                                                                                                                match #[lang = "into_iter"](#[lang = "range_inclusive_new"](b'a',
                                                                                                                                                                                                    b'z')) {
                                                                                                                                                                                        mut iter =>
                                                                                                                                                                                            loop {
                                                                                                                                                                                                    match #[lang = "next"](&mut iter) {
                                                                                                                                                                                                            #[lang = "None"] {} => break,
                                                                                                                                                                                                            #[lang = "Some"] {  0: nc } => {
                                                                                                                                                                                                                unsafe { o.as_bytes_mut()[i] = nc; }
                                                                                                                                                                                                                if words.contains(&o)
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                words.remove(&o);
                                                                                                                                                                                                                                if o == end_word { return dist; } q.push_back(o.clone()); }
                                                                                                                                                                                                                        } } }, };
                                                                                                                                                                                            _t
                                                                                                                                                                                        } } } }, };
                                                                                                                                                _t
                                                                                                                                            } } } }, };
                                                                                                    _t
                                                                                                } } else { break; } } 0 }
                                                                }
                                                                fn main() {
                                                                        let words =
                                                                            <[_]>::into_vec(
                                                                                #[rustc_box]
                                                                                ::alloc::boxed::Box::new(["hot", "dog", "dot"]));
                                                                        let d =
                                                                            S2::ladder_length("hot".to_string(), "dog".to_string(),
                                                                                words.into_iter().map(|x| { x.to_string() }).collect());
                                                                        {
                                                                                ::std::io::_print(<#[lang = "format_arguments"]>::new_v1(&["",
                                                                                                    "\n"], &[<#[lang = "format_argument"]>::new_display(&d)]));
                                                                            };
                                                                    }
