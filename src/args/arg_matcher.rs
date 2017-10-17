// Std
use std::collections::hash_map::{Entry, Iter};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::ops::Deref;
use std::mem;

// Internal
use args::{ArgMatches, MatchedArg, SubCommand};
use args::AnyArg;
use args::settings::ArgSettings;

#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub struct ArgMatcher<'a>(pub ArgMatches<'a>);

impl<'a> Default for ArgMatcher<'a> {
    fn default() -> Self { ArgMatcher(ArgMatches::default()) }
}

impl<'a> ArgMatcher<'a> {
    pub fn new() -> Self { ArgMatcher::default() }

    // TODO: maybe this would work better as a while loop
    // than as a recursive call, so that we can push subcommands
    // onto a stack and gather values from them as we go, then
    // pop the stack and set values as needed
    pub fn get_global_values(&mut self, global_arg_vec : &Vec<&'a str>, vals_map: &mut HashMap<&'a str, Vec<OsString>>) {
        for global_arg in global_arg_vec.iter() {
            let vals: Vec<_> = if let Some(ma) = self.get(global_arg) {
                ma.vals.clone()
            } else {
                debugln!("ArgMatcher::propagate: arg wasn't used");
                //return; // we can't return early because some subcommand might
                // TODO: can we start returning early again if we
                Vec::new()
            };
            vals_map.insert(global_arg, vals);
        }
    }


    // pub fn propagate_globals(&mut self, global_arg_vec: Vec<&'a str>) {
    //     debugln!("ArgMatcher::propagate: arg={}", arg);
    //     // we need to decide which subcommands were actually used by asking
    //     // each successive subcommand for the next subcommand
    //     let mut subcommand_stack : Vec<Box<SubCommand>> = vec![];


    //     //let mut am : &mut ArgMatcher;
    //     if let Some(ref mut sc) = self.0.subcommand {
    //         let mut found_subcommand = sc.clone();
    //         let mut unboxed_sub : SubCommand = *found_subcommand;
    //         subcommand_stack.push(Box::new(unboxed_sub.clone()));
    //         let mut option_sub = unboxed_sub.matches.subcommand().1;
    //         while let Some(ref mut new_arg_matches) = option_sub {
    //             if let Some(ref new_subcommand) = new_arg_matches.subcommand {
    //                 subcommand_stack.push(Box::new(*new_subcommand.clone()));
    //                 option_sub = Some(&new_subcommand.matches);
    //             }
    //         }
    //     }
    //     // TODO make a new matcher and propagate 
    //     for boxed_sub in subcommand_stack.iter() {
    //         let mut unboxed_sub = &*boxed_sub;
    //         let mut ix: usize = 0; 
    //         for global_arg in global_arg_vec.iter() {
    //             let vals = &vals_vec[ix];
                
    //             let sma = (unboxed_sub).matches.args.entry(global_arg).or_insert_with(|| {
    //                     let mut gma = MatchedArg::new();
    //                     gma.occurs += 1;
    //                     if !vals.is_empty() {
    //                         gma.vals = *vals.clone();
    //                     }
    //                     gma
    //             });
    //             if sma.vals.is_empty() {
    //                     sma.vals = *vals.clone();
    //             }
    //             ix += 1; 
    //         }

    //         //let mut am = //&mut ArgMatcher(mem::replace(&mut sc.matches, ArgMatches::new()));
    //         let mut am = ArgMatcher(ArgMatches::new());
    //         mem::swap(&mut am.0, &mut unboxed_sub.matches);
    //     }

    //     // for arg in global_arg_vec {

    //     //     if let Some(ref mut sc) = self.0.subcommand {
    //     //         {
    //     //             let sma = (*sc).matches.args.entry(arg).or_insert_with(|| {
    //     //                 let mut gma = MatchedArg::new();
    //     //                 gma.occurs += 1;
    //     //                 if !vals.is_empty() {
    //     //                     gma.vals = vals.clone();
    //     //                 }
    //     //                 gma
    //     //             });
    //     //             if sma.vals.is_empty() {
    //     //                 sma.vals = vals.clone();
    //     //             }
    //     //         }
    //     //         let mut am = ArgMatcher(mem::replace(&mut sc.matches, ArgMatches::new()));
    //     //         am.propagate(global_arg_vec.clone());
    //     //         mem::swap(&mut am.0, &mut sc.matches);
    //     //     } else {
    //     //         debugln!("ArgMatcher::propagate: Subcommand wasn't used");
    //     //     }
    //     // }
    // }

    pub fn get_mut(&mut self, arg: &str) -> Option<&mut MatchedArg> { self.0.args.get_mut(arg) }

    pub fn get(&self, arg: &str) -> Option<&MatchedArg> { self.0.args.get(arg) }

    pub fn remove(&mut self, arg: &str) { self.0.args.remove(arg); }

    pub fn remove_all(&mut self, args: &[&str]) {
        for &arg in args {
            self.0.args.remove(arg);
        }
    }

    pub fn insert(&mut self, name: &'a str) { self.0.args.insert(name, MatchedArg::new()); }

    pub fn contains(&self, arg: &str) -> bool { self.0.args.contains_key(arg) }

    pub fn is_empty(&self) -> bool { self.0.args.is_empty() }

    pub fn usage(&mut self, usage: String) { self.0.usage = Some(usage); }

    pub fn arg_names(&'a self) -> Vec<&'a str> { self.0.args.keys().map(Deref::deref).collect() }

    pub fn entry(&mut self, arg: &'a str) -> Entry<&'a str, MatchedArg> { self.0.args.entry(arg) }

    pub fn subcommand(&mut self, sc: SubCommand<'a>) { self.0.subcommand = Some(Box::new(sc)); }

    pub fn subcommand_name(&self) -> Option<&str> { self.0.subcommand_name() }

    pub fn iter(&self) -> Iter<&str, MatchedArg> { self.0.args.iter() }

    pub fn inc_occurrence_of(&mut self, arg: &'a str) {
        debugln!("ArgMatcher::inc_occurrence_of: arg={}", arg);
        if let Some(a) = self.get_mut(arg) {
            a.occurs += 1;
            return;
        }
        debugln!("ArgMatcher::inc_occurrence_of: first instance");
        self.insert(arg);
    }

    pub fn inc_occurrences_of(&mut self, args: &[&'a str]) {
        debugln!("ArgMatcher::inc_occurrences_of: args={:?}", args);
        for arg in args {
            self.inc_occurrence_of(arg);
        }
    }

    pub fn add_val_to(&mut self, arg: &'a str, val: &OsStr) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            occurs: 0,
            vals: Vec::with_capacity(1),
        });
        // let len = ma.vals.len() + 1;
        ma.vals.push(val.to_owned());
    }

    pub fn needs_more_vals<'b, A>(&self, o: &A) -> bool
        where A: AnyArg<'a, 'b>
    {
        debugln!("ArgMatcher::needs_more_vals: o={}", o.name());
        if let Some(ma) = self.get(o.name()) {
            if let Some(num) = o.num_vals() {
                debugln!("ArgMatcher::needs_more_vals: num_vals...{}", num);
                return if o.is_set(ArgSettings::Multiple) {
                    ((ma.vals.len() as u64) % num) != 0
                } else {
                    num != (ma.vals.len() as u64)
                };
            } else if let Some(num) = o.max_vals() {
                debugln!("ArgMatcher::needs_more_vals: max_vals...{}", num);
                return !((ma.vals.len() as u64) > num);
            } else if o.min_vals().is_some() {
                debugln!("ArgMatcher::needs_more_vals: min_vals...true");
                return true;
            }
            return o.is_set(ArgSettings::Multiple);
        }
        true
    }
}

impl<'a> Into<ArgMatches<'a>> for ArgMatcher<'a> {
    fn into(self) -> ArgMatches<'a> { self.0 }
}
