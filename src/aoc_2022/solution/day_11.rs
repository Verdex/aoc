
struct Entity {
    items : Vec<u128>,
    op : fn(u128) -> u128,
    pred : fn(u128) -> bool,
    pred_suc : usize,
    pred_fail : usize,
    total: u128,
}

fn round(entities : &mut Vec<Entity>) {
    for i in 0..entities.len() {
        while entities[i].items.len() != 0 {
            let item = entities[i].items.remove(0);
            let new = (entities[i].op)(item) / 3;
            entities[i].total += 1;
            if (entities[i].pred)(new) {
                let target = entities[i].pred_suc;
                entities[target].items.push(new);
            }
            else {
                let target = entities[i].pred_fail;
                entities[target].items.push(new);
            }
        }
    }
}

fn round_v2(entities : &mut Vec<Entity>) {
    for i in 0..entities.len() {
        while entities[i].items.len() != 0 {
            let item = entities[i].items.remove(0);
            let new = (entities[i].op)(item) % (11 * 19 * 5 * 2 * 13 * 7 * 3 * 17);
            entities[i].total += 1;
            if (entities[i].pred)(new) {
                let target = entities[i].pred_suc;
                entities[target].items.push(new);
            }
            else {
                let target = entities[i].pred_fail;
                entities[target].items.push(new);
            }
        }
    }
}

fn display_items(entities : &[Entity]) {
    for (i, x) in entities.iter().enumerate() {
        println!("{} : {:?} : {}", i, x.items, x.total);
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    /*let mut entities 
        = vec! [ Entity { items: vec![79, 98], op : |x| x * 19, pred: |x| x % 23 == 0, pred_suc: 2, pred_fail: 3, total: 0}
               , Entity { items: vec![54, 65, 75, 74], op : |x| x + 6, pred: |x| x % 19 == 0, pred_suc: 2, pred_fail: 0, total: 0}
               , Entity { items: vec![79, 60, 97], op : |x| x * x, pred: |x| x % 13 == 0, pred_suc: 1, pred_fail: 3, total: 0}
               , Entity { items: vec![74], op : |x| x + 3, pred: |x| x % 17 == 0, pred_suc: 0, pred_fail: 1, total: 0}
               ];*/
    let mut entities 
        = vec! [ Entity { items: vec![97, 81, 57, 57, 91, 61], op : |x| x * 7, pred: |x| x % 11 == 0, pred_suc: 5, pred_fail: 6, total: 0}
               , Entity { items: vec![88, 62, 68, 90], op : |x| x * 17, pred: |x| x % 19 == 0, pred_suc: 4, pred_fail: 2, total: 0}
               , Entity { items: vec![74, 87], op : |x| x + 2, pred: |x| x % 5 == 0, pred_suc: 7, pred_fail: 4, total: 0}
               , Entity { items: vec![53, 81, 60, 87, 90, 99, 75], op : |x| x + 1, pred: |x| x % 2 == 0, pred_suc: 2, pred_fail: 1, total: 0}
               , Entity { items: vec![57], op : |x| x + 6, pred: |x| x % 13 == 0, pred_suc: 7, pred_fail: 0, total: 0}
               , Entity { items: vec![54, 84, 91, 55, 59, 72, 75, 70], op : |x| x * x, pred: |x| x % 7 == 0, pred_suc: 6, pred_fail: 3, total: 0}
               , Entity { items: vec![95, 79, 79, 68, 78], op : |x| x + 3, pred: |x| x % 3 == 0, pred_suc: 1, pred_fail: 3, total: 0}
               , Entity { items: vec![61, 97, 67], op : |x| x + 4, pred: |x| x % 17 == 0, pred_suc: 0, pred_fail: 5, total: 0}
               ];


    for _ in 0..20 {
        round(&mut entities);
    }
    
    let mut totals = entities.iter().map(|x| x.total).collect::<Vec<_>>();
    totals.sort();
    totals.reverse();

    let result = totals[0] * totals[1];

    println!("2022 day 11:1 = {}", result);
}

#[allow(dead_code)]
pub fn solve_2() {
    /*let mut entities 
        = vec! [ Entity { items: vec![79, 98], op : |x| x * 19, pred: |x| x % 23 == 0, pred_suc: 2, pred_fail: 3, total: 0}
               , Entity { items: vec![54, 65, 75, 74], op : |x| x + 6, pred: |x| x % 19 == 0, pred_suc: 2, pred_fail: 0, total: 0}
               , Entity { items: vec![79, 60, 97], op : |x| x * x, pred: |x| x % 13 == 0, pred_suc: 1, pred_fail: 3, total: 0}
               , Entity { items: vec![74], op : |x| x + 3, pred: |x| x % 17 == 0, pred_suc: 0, pred_fail: 1, total: 0}
               ];*/
    let mut entities 
        = vec! [ Entity { items: vec![97, 81, 57, 57, 91, 61], op : |x| x * 7, pred: |x| x % 11 == 0, pred_suc: 5, pred_fail: 6, total: 0}
               , Entity { items: vec![88, 62, 68, 90], op : |x| x * 17, pred: |x| x % 19 == 0, pred_suc: 4, pred_fail: 2, total: 0}
               , Entity { items: vec![74, 87], op : |x| x + 2, pred: |x| x % 5 == 0, pred_suc: 7, pred_fail: 4, total: 0}
               , Entity { items: vec![53, 81, 60, 87, 90, 99, 75], op : |x| x + 1, pred: |x| x % 2 == 0, pred_suc: 2, pred_fail: 1, total: 0}
               , Entity { items: vec![57], op : |x| x + 6, pred: |x| x % 13 == 0, pred_suc: 7, pred_fail: 0, total: 0}
               , Entity { items: vec![54, 84, 91, 55, 59, 72, 75, 70], op : |x| x * x, pred: |x| x % 7 == 0, pred_suc: 6, pred_fail: 3, total: 0}
               , Entity { items: vec![95, 79, 79, 68, 78], op : |x| x + 3, pred: |x| x % 3 == 0, pred_suc: 1, pred_fail: 3, total: 0}
               , Entity { items: vec![61, 97, 67], op : |x| x + 4, pred: |x| x % 17 == 0, pred_suc: 0, pred_fail: 5, total: 0}
               ];


    for _ in 0..10_000 {
        round_v2(&mut entities);
    }
    
    let mut totals = entities.iter().map(|x| x.total).collect::<Vec<_>>();
    totals.sort();
    totals.reverse();

    let result = totals[0] * totals[1];

    println!("2022 day 11:2 = {}", result);
}

