use crate::runtime::{*};
use std::sync::atomic::{AtomicBool, Ordering};

// Precomps
// --------

pub struct PrecompFuns {
  pub visit: VisitFun,
  pub apply: ApplyFun,
}

pub struct Precomp {
  pub id: u64,
  pub name: &'static str,
  pub smap: &'static [bool],
  pub funs: Option<PrecompFuns>,
}

pub const STRING_NIL : u64 = 0;
pub const STRING_CONS : u64 = 1;
pub const BOTH : u64 = 2;
pub const KIND_TERM_CT0 : u64 = 3;
pub const KIND_TERM_CT1 : u64 = 4;
pub const KIND_TERM_CT2 : u64 = 5;
pub const KIND_TERM_CT3 : u64 = 6;
pub const KIND_TERM_CT4 : u64 = 7;
pub const KIND_TERM_CT5 : u64 = 8;
pub const KIND_TERM_CT6 : u64 = 9;
pub const KIND_TERM_CT7 : u64 = 10;
pub const KIND_TERM_CT8 : u64 = 11;
pub const KIND_TERM_CT9 : u64 = 12;
pub const KIND_TERM_CTA : u64 = 13;
pub const KIND_TERM_CTB : u64 = 14;
pub const KIND_TERM_CTC : u64 = 15;
pub const KIND_TERM_CTD : u64 = 16;
pub const KIND_TERM_CTE : u64 = 17;
pub const KIND_TERM_CTF : u64 = 18;
pub const KIND_TERM_CTG : u64 = 19;
pub const KIND_TERM_U60 : u64 = 20;
pub const KIND_TERM_F60 : u64 = 21;
pub const U60_IF : u64 = 22;
pub const U60_SWAP : u64 = 23;
pub const HVM_LOG : u64 = 24;
pub const HVM_QUERY : u64 = 25;
pub const HVM_PRINT : u64 = 26;
pub const HVM_SLEEP : u64 = 27;
pub const HVM_STORE : u64 = 28;
pub const HVM_LOAD : u64 = 29;
pub const _String_nil_ : u64 = 0;
pub const _String_cons_ : u64 = 1;
pub const _Both_ : u64 = 2;
pub const _Kind_Term_ct0_ : u64 = 3;
pub const _Kind_Term_ct1_ : u64 = 4;
pub const _Kind_Term_ct2_ : u64 = 5;
pub const _Kind_Term_ct3_ : u64 = 6;
pub const _Kind_Term_ct4_ : u64 = 7;
pub const _Kind_Term_ct5_ : u64 = 8;
pub const _Kind_Term_ct6_ : u64 = 9;
pub const _Kind_Term_ct7_ : u64 = 10;
pub const _Kind_Term_ct8_ : u64 = 11;
pub const _Kind_Term_ct9_ : u64 = 12;
pub const _Kind_Term_ctA_ : u64 = 13;
pub const _Kind_Term_ctB_ : u64 = 14;
pub const _Kind_Term_ctC_ : u64 = 15;
pub const _Kind_Term_ctD_ : u64 = 16;
pub const _Kind_Term_ctE_ : u64 = 17;
pub const _Kind_Term_ctF_ : u64 = 18;
pub const _Kind_Term_ctG_ : u64 = 19;
pub const _Kind_Term_u60_ : u64 = 20;
pub const _Kind_Term_f60_ : u64 = 21;
pub const _U60_if_ : u64 = 22;
pub const _U60_swap_ : u64 = 23;
pub const _HVM_log_ : u64 = 24;
pub const _HVM_query_ : u64 = 25;
pub const _HVM_print_ : u64 = 26;
pub const _HVM_sleep_ : u64 = 27;
pub const _HVM_store_ : u64 = 28;
pub const _HVM_load_ : u64 = 29;
pub const _Main_ : u64 = 30;
pub const _Show_ : u64 = 31;
pub const _IsNone_ : u64 = 32;
pub const _Some_ : u64 = 33;
pub const _True_ : u64 = 34;
pub const _False_ : u64 = 35;
pub const _Bool_eql_ : u64 = 36;
pub const _None_ : u64 = 37;

pub const PRECOMP : &[Precomp] = &[
  Precomp {
    id: STRING_NIL,
    name: "String.nil",
    smap: &[false; 0],
    funs: None,
  },
  Precomp {
    id: STRING_CONS,
    name: "String.cons",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: BOTH,
    name: "Both",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT0,
    name: "Kind.Term.ct0",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT1,
    name: "Kind.Term.ct1",
    smap: &[false; 3],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT2,
    name: "Kind.Term.ct2",
    smap: &[false; 4],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT3,
    name: "Kind.Term.ct3",
    smap: &[false; 5],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT4,
    name: "Kind.Term.ct4",
    smap: &[false; 6],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT5,
    name: "Kind.Term.ct5",
    smap: &[false; 7],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT6,
    name: "Kind.Term.ct6",
    smap: &[false; 8],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT7,
    name: "Kind.Term.ct7",
    smap: &[false; 9],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT8,
    name: "Kind.Term.ct8",
    smap: &[false; 10],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CT9,
    name: "Kind.Term.ct9",
    smap: &[false; 11],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTA,
    name: "Kind.Term.ctA",
    smap: &[false; 12],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTB,
    name: "Kind.Term.ctB",
    smap: &[false; 13],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTC,
    name: "Kind.Term.ctC",
    smap: &[false; 14],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTD,
    name: "Kind.Term.ctD",
    smap: &[false; 15],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTE,
    name: "Kind.Term.ctE",
    smap: &[false; 16],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTF,
    name: "Kind.Term.ctF",
    smap: &[false; 17],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_CTG,
    name: "Kind.Term.ctG",
    smap: &[false; 18],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_U60,
    name: "Kind.Term.u60",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: KIND_TERM_F60,
    name: "Kind.Term.f60",
    smap: &[false; 2],
    funs: None,
  },
  Precomp {
    id: U60_IF,
    name: "U60.if",
    smap: &[true, false, false],
    funs: Some(PrecompFuns {
      visit: u60_if_visit,
      apply: u60_if_apply,
    }),
  },
  Precomp {
    id: U60_SWAP,
    name: "U60.swap",
    smap: &[true, false, false],
    funs: Some(PrecompFuns {
      visit: u60_swap_visit,
      apply: u60_swap_apply,
    }),
  },
  Precomp {
    id: HVM_LOG,
    name: "HVM.log",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_log_visit,
      apply: hvm_log_apply,
    }),
  },
  Precomp {
    id: HVM_QUERY,
    name: "HVM.query",
    smap: &[false; 1],
    funs: Some(PrecompFuns {
      visit: hvm_query_visit,
      apply: hvm_query_apply,
    }),
  },
  Precomp {
    id: HVM_PRINT,
    name: "HVM.print",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_print_visit,
      apply: hvm_print_apply,
    }),
  },
  Precomp {
    id: HVM_SLEEP,
    name: "HVM.sleep",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_sleep_visit,
      apply: hvm_sleep_apply,
    }),
  },
  Precomp {
    id: HVM_STORE,
    name: "HVM.store",
    smap: &[false; 3],
    funs: Some(PrecompFuns {
      visit: hvm_store_visit,
      apply: hvm_store_apply,
    }),
  },
  Precomp {
    id: HVM_LOAD,
    name: "HVM.load",
    smap: &[false; 2],
    funs: Some(PrecompFuns {
      visit: hvm_load_visit,
      apply: hvm_load_apply,
    }),
  },
  Precomp {
    id: _Main_,
    name: "Main",
    smap: &[],
    funs: Some(PrecompFuns {
      visit: _Main__visit,
      apply: _Main__apply,
    }),
  },
  Precomp {
    id: _Show_,
    name: "Show",
    smap: &[false],
    funs: Some(PrecompFuns {
      visit: _Show__visit,
      apply: _Show__apply,
    }),
  },
  Precomp {
    id: _IsNone_,
    name: "IsNone",
    smap: &[false],
    funs: Some(PrecompFuns {
      visit: _IsNone__visit,
      apply: _IsNone__apply,
    }),
  },
  Precomp {
    id: _Some_,
    name: "Some",
    smap: &[],
    funs: Some(PrecompFuns {
      visit: _Some__visit,
      apply: _Some__apply,
    }),
  },
  Precomp {
    id: _True_,
    name: "True",
    smap: &[],
    funs: Some(PrecompFuns {
      visit: _True__visit,
      apply: _True__apply,
    }),
  },
  Precomp {
    id: _False_,
    name: "False",
    smap: &[],
    funs: Some(PrecompFuns {
      visit: _False__visit,
      apply: _False__apply,
    }),
  },
  Precomp {
    id: _Bool_eql_,
    name: "Bool.eql",
    smap: &[false, false],
    funs: Some(PrecompFuns {
      visit: _Bool_eql__visit,
      apply: _Bool_eql__apply,
    }),
  },
  Precomp {
    id: _None_,
    name: "None",
    smap: &[],
    funs: Some(PrecompFuns {
      visit: _None__visit,
      apply: _None__apply,
    }),
  },
];

pub const PRECOMP_COUNT : u64 = PRECOMP.len() as u64;

// Ul0.if (cond: Term) (if_t: Term) (if_f: Term)
// ---------------------------------------------

#[inline(always)]
pub fn u60_if_visit(ctx: ReduceCtx) -> bool {
  if is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, 1));
    *ctx.cont = goup;
    *ctx.host = get_loc(ctx.term, 0);
    return true;
  }
}

#[inline(always)]
pub fn u60_if_apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  let arg2 = load_arg(ctx.heap, ctx.term, 2);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == U60) {
    if (get_num(arg0) == 0) {
      inc_cost(ctx.heap, ctx.tid);
      let done = arg2;
      link(ctx.heap, *ctx.host, done);
      collect(ctx.heap, &ctx.prog.aris, ctx.tid, arg1);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    } else {
      inc_cost(ctx.heap, ctx.tid);
      let done = arg1;
      link(ctx.heap, *ctx.host, done);
      collect(ctx.heap, &ctx.prog.aris, ctx.tid, arg2);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    }
  }
  return false;
}

// U60.swap (cond: Term) (pair: Term)
// ----------------------------------

#[inline(always)]
pub fn u60_swap_visit(ctx: ReduceCtx) -> bool {
  if is_whnf(load_arg(ctx.heap, ctx.term, 0)) {
    return false;
  } else {
    let goup = ctx.redex.insert(ctx.tid, new_redex(*ctx.host, *ctx.cont, 1));
    *ctx.cont = goup;
    *ctx.host = get_loc(ctx.term, 0);
    return true;
  }
}

#[inline(always)]
pub fn u60_swap_apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  let arg2 = load_arg(ctx.heap, ctx.term, 2);
  if get_tag(arg0) == SUP {
    fun::superpose(ctx.heap, &ctx.prog.aris, ctx.tid, *ctx.host, ctx.term, arg0, 0);
  }
  if (get_tag(arg0) == U60) {
    if (get_num(arg0) == 0) {
      inc_cost(ctx.heap, ctx.tid);
      let ctr_0 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, ctr_0 + 0, arg1);
      link(ctx.heap, ctr_0 + 1, arg2);
      let done = Ctr(BOTH, ctr_0);
      link(ctx.heap, *ctx.host, done);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    } else {
      inc_cost(ctx.heap, ctx.tid);
      let ctr_0 = alloc(ctx.heap, ctx.tid, 2);
      link(ctx.heap, ctr_0 + 0, arg2);
      link(ctx.heap, ctr_0 + 1, arg1);
      let done = Ctr(BOTH, ctr_0);
      link(ctx.heap, *ctx.host, done);
      free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
      return true;
    }
  }
  return false;
}

// HVM.log (term: Term)
// --------------------

fn hvm_log_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_log_apply(ctx: ReduceCtx) -> bool {
  normalize(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0), false);
  let code = crate::language::readback::as_code(ctx.heap, ctx.prog, get_loc(ctx.term, 0));
  println!("{}", code);
  link(ctx.heap, *ctx.host, load_arg(ctx.heap, ctx.term, 1));
  collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_ptr(ctx.heap, get_loc(ctx.term, 0)));
  free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
  return true;
}

// HVM.query (cont: String -> Term)
// --------------------------------

fn hvm_query_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_query_apply(ctx: ReduceCtx) -> bool {
  fn read_input() -> String {
    use std::io::{stdin,stdout,Write};
    let mut input = String::new();
    stdin().read_line(&mut input).expect("string");
    if let Some('\n') = input.chars().next_back() { input.pop(); }
    if let Some('\r') = input.chars().next_back() { input.pop(); }
    return input;
  }
  let cont = load_arg(ctx.heap, ctx.term, 0);
  let text = make_string(ctx.heap, ctx.tid, &read_input());
  let app0 = alloc(ctx.heap, ctx.tid, 2);
  link(ctx.heap, app0 + 0, cont);
  link(ctx.heap, app0 + 1, text);
  free(ctx.heap, 0, get_loc(ctx.term, 0), 1);
  let done = App(app0);
  link(ctx.heap, *ctx.host, done);
  return true;
}

// HVM.print (text: String) (cont: Term)
// -----------------------------------------------

fn hvm_print_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_print_apply(ctx: ReduceCtx) -> bool {
  //normalize(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0), false);
  if let Some(text) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0)) {
    println!("{}", text);
  }
  link(ctx.heap, *ctx.host, load_arg(ctx.heap, ctx.term, 1));
  collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_ptr(ctx.heap, get_loc(ctx.term, 0)));
  free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
  return true;
}

// HVM.sleep (time: U60) (cont: Term)
// ----------------------------------

fn hvm_sleep_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_sleep_apply(ctx: ReduceCtx) -> bool {
  let time = reduce(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0), true, false);
  std::thread::sleep(std::time::Duration::from_nanos(get_num(time)));
  link(ctx.heap, *ctx.host, load_ptr(ctx.heap, get_loc(ctx.term, 1)));
  free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 2);
  return true;
}

// HVM.store (key: String) (val: String) (cont: Term)
// --------------------------------------------------

fn hvm_store_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_store_apply(ctx: ReduceCtx) -> bool {
  if let Some(key) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0)) {
    if let Some(val) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 1)) {
      if std::fs::write(key, val).is_ok() {
        //let app0 = alloc(ctx.heap, ctx.tid, 2);
        //link(ctx.heap, app0 + 0, cont);
        //link(ctx.heap, app0 + 1, U6O(0));
        //free(ctx.heap, 0, get_loc(ctx.term, 0), 2);
        let done = load_arg(ctx.heap, ctx.term, 2);
        link(ctx.heap, *ctx.host, done);
        collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_arg(ctx.heap, ctx.term, 0));
        collect(ctx.heap, &ctx.prog.aris, ctx.tid, load_arg(ctx.heap, ctx.term, 1));
        free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 3);
        return true;
      }
    }
  }
  println!("Runtime failure on: {}", show_at(ctx.heap, ctx.prog, *ctx.host, &[]));
  std::process::exit(0);
}

// HVM.load (key: String) (cont: String -> Term)
// ---------------------------------------------

fn hvm_load_visit(ctx: ReduceCtx) -> bool {
  return false;
}

fn hvm_load_apply(ctx: ReduceCtx) -> bool {
  if let Some(key) = crate::language::readback::as_string(ctx.heap, ctx.prog, &[ctx.tid], get_loc(ctx.term, 0)) {
    if let Ok(file) = std::fs::read(key) {
      if let Ok(file) = std::str::from_utf8(&file) {
        let cont = load_arg(ctx.heap, ctx.term, 1); 
        let text = make_string(ctx.heap, ctx.tid, file);
        let app0 = alloc(ctx.heap, ctx.tid, 2);
        link(ctx.heap, app0 + 0, cont);
        link(ctx.heap, app0 + 1, text);
        free(ctx.heap, 0, get_loc(ctx.term, 0), 2);
        let done = App(app0);
        link(ctx.heap, *ctx.host, done);
        return true;
      }
    }
  }
  println!("Runtime failure on: {}", show_at(ctx.heap, ctx.prog, *ctx.host, &[]));
  std::process::exit(0);
}

#[inline(always)]
pub fn _Main__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Main__apply(ctx: ReduceCtx) -> bool {
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cal_1 = get_loc(ctx.term, 0)/*reuse:0*/;
    let cal_2 = alloc(ctx.heap, ctx.tid, 0);
    let app_0 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_0 + 0, Fun(_Some_, cal_1));
    link(ctx.heap, app_0 + 1, Fun(_True_, cal_2));
    let cal_3 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_3 + 0, App(app_0));
    let cal_4 = alloc(ctx.heap, ctx.tid, 1);
    link(ctx.heap, cal_4 + 0, Fun(_IsNone_, cal_3));
    let done = Fun(_Show_, cal_4);
    link(ctx.heap, *ctx.host, done);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _Show__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Show__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let ctr_2 = alloc(ctx.heap, ctx.tid, 0);
    let ctr_3 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_3 + 0, U6O(101));
    link(ctx.heap, ctr_3 + 1, Ctr(_String_nil_, ctr_2));
    let ctr_4 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_4 + 0, U6O(117));
    link(ctx.heap, ctr_4 + 1, Ctr(_String_cons_, ctr_3));
    let ctr_5 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_5 + 0, U6O(114));
    link(ctx.heap, ctr_5 + 1, Ctr(_String_cons_, ctr_4));
    let ctr_6 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_6 + 0, U6O(84));
    link(ctx.heap, ctr_6 + 1, Ctr(_String_cons_, ctr_5));
    let app_1 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_1 + 0, arg0);
    link(ctx.heap, app_1 + 1, Ctr(_String_cons_, ctr_6));
    let ctr_7 = alloc(ctx.heap, ctx.tid, 0);
    let ctr_8 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_8 + 0, U6O(101));
    link(ctx.heap, ctr_8 + 1, Ctr(_String_nil_, ctr_7));
    let ctr_9 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_9 + 0, U6O(115));
    link(ctx.heap, ctr_9 + 1, Ctr(_String_cons_, ctr_8));
    let ctr_10 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_10 + 0, U6O(108));
    link(ctx.heap, ctr_10 + 1, Ctr(_String_cons_, ctr_9));
    let ctr_11 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_11 + 0, U6O(97));
    link(ctx.heap, ctr_11 + 1, Ctr(_String_cons_, ctr_10));
    let ctr_12 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, ctr_12 + 0, U6O(70));
    link(ctx.heap, ctr_12 + 1, Ctr(_String_cons_, ctr_11));
    let app_0 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_0 + 0, App(app_1));
    link(ctx.heap, app_0 + 1, Ctr(_String_cons_, ctr_12));
    let done = App(app_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _IsNone__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _IsNone__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cal_2 = alloc(ctx.heap, ctx.tid, 0);
    let app_1 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_1 + 0, arg0);
    link(ctx.heap, app_1 + 1, Fun(_True_, cal_2));
    let lam_3 = alloc(ctx.heap, ctx.tid, 2);
    let cal_4 = alloc(ctx.heap, ctx.tid, 0);
    link(ctx.heap, lam_3 + 0, Era());
    link(ctx.heap, lam_3 + 1, Fun(_False_, cal_4));
    let app_0 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_0 + 0, App(app_1));
    link(ctx.heap, app_0 + 1, Lam(lam_3));
    let done = App(app_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 1);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _Some__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Some__apply(ctx: ReduceCtx) -> bool {
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let lam_0 = alloc(ctx.heap, ctx.tid, 2);
    let lam_1 = alloc(ctx.heap, ctx.tid, 2);
    let lam_2 = alloc(ctx.heap, ctx.tid, 2);
    let app_3 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_3 + 0, Var(lam_2));
    link(ctx.heap, app_3 + 1, Var(lam_0));
    link(ctx.heap, lam_2 + 1, App(app_3));
    link(ctx.heap, lam_1 + 0, Era());
    link(ctx.heap, lam_1 + 1, Lam(lam_2));
    link(ctx.heap, lam_0 + 1, Lam(lam_1));
    let done = Lam(lam_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 0);
    return false;
  }
  return false;
}

#[inline(always)]
pub fn _True__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _True__apply(ctx: ReduceCtx) -> bool {
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let lam_0 = alloc(ctx.heap, ctx.tid, 2);
    let lam_1 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, lam_1 + 0, Era());
    link(ctx.heap, lam_1 + 1, Var(lam_0));
    link(ctx.heap, lam_0 + 1, Lam(lam_1));
    let done = Lam(lam_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 0);
    return false;
  }
  return false;
}

#[inline(always)]
pub fn _False__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _False__apply(ctx: ReduceCtx) -> bool {
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let lam_0 = alloc(ctx.heap, ctx.tid, 2);
    let lam_1 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, lam_1 + 1, Var(lam_1));
    link(ctx.heap, lam_0 + 0, Era());
    link(ctx.heap, lam_0 + 1, Lam(lam_1));
    let done = Lam(lam_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 0);
    return false;
  }
  return false;
}

#[inline(always)]
pub fn _Bool_eql__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _Bool_eql__apply(ctx: ReduceCtx) -> bool {
  let arg0 = load_arg(ctx.heap, ctx.term, 0);
  let arg1 = load_arg(ctx.heap, ctx.term, 1);
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let cpy_0 = arg1;
    let dp0_1;
    let dp1_2;
    if get_tag(cpy_0) == U60 || get_tag(cpy_0) == F60 {
      inc_cost(ctx.heap, ctx.tid);
      dp0_1 = cpy_0;
      dp1_2 = cpy_0;
    } else {
      let col_3 = gen_dup(ctx.heap, ctx.tid);
      let dup_4 = alloc(ctx.heap, ctx.tid, 3);
    link(ctx.heap, dup_4 + 0, Era());
    link(ctx.heap, dup_4 + 1, Era());
      link(ctx.heap, dup_4 + 2, cpy_0);
      dp0_1 = Dp0(col_3, dup_4);
      dp1_2 = Dp1(col_3, dup_4);
    }
    let app_6 = get_loc(ctx.term, 0)/*reuse:2*/;
    link(ctx.heap, app_6 + 0, arg0);
    link(ctx.heap, app_6 + 1, dp0_1);
    let cal_9 = alloc(ctx.heap, ctx.tid, 0);
    let app_8 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_8 + 0, dp1_2);
    link(ctx.heap, app_8 + 1, Fun(_False_, cal_9));
    let cal_10 = alloc(ctx.heap, ctx.tid, 0);
    let app_7 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_7 + 0, App(app_8));
    link(ctx.heap, app_7 + 1, Fun(_True_, cal_10));
    let app_5 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, app_5 + 0, App(app_6));
    link(ctx.heap, app_5 + 1, App(app_7));
    let done = App(app_5);
    link(ctx.heap, *ctx.host, done);
    return true;
  }
  return false;
}

#[inline(always)]
pub fn _None__visit(ctx: ReduceCtx) -> bool {
  return false;
}

#[inline(always)]
pub fn _None__apply(ctx: ReduceCtx) -> bool {
  if true {
    inc_cost(ctx.heap, ctx.tid);
    let lam_0 = alloc(ctx.heap, ctx.tid, 2);
    let lam_1 = alloc(ctx.heap, ctx.tid, 2);
    link(ctx.heap, lam_1 + 0, Era());
    link(ctx.heap, lam_1 + 1, Var(lam_0));
    link(ctx.heap, lam_0 + 1, Lam(lam_1));
    let done = Lam(lam_0);
    link(ctx.heap, *ctx.host, done);
    free(ctx.heap, ctx.tid, get_loc(ctx.term, 0), 0);
    return false;
  }
  return false;
}

