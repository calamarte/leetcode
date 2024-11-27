fn main() {
    let (gas, cost) = (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
    // let (gas, cost) = (vec![2, 3, 4], vec![3, 4, 3]);
    // let (gas, cost) = (vec![5, 8, 2, 8], vec![6, 5, 6, 6]);
    // let (gas, cost) = (
    //     vec![
    //         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //     ],
    //     vec![
    //         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //         1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //     ],
    // );

    let result = can_complete_circuit_b(gas, cost);

    println!("result -> {result}");
}

fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut tank = 0;
    let mut start = 0;
    let mut quota = 0;

    println!("{gas:?}\n{cost:?}");

    for i in 0..gas.len() {
        tank += gas[i] - cost[i]; // Where can we start?
        quota += gas[i] - cost[i]; // We sum differences to know if is possible to do all the circuit

        println!("tank -> {tank}\nquot -> {quota}");

        if tank < 0 {
            start = i + 1;
            println!("reset tank, start -> {start}");
            tank = 0;
        }
    }

    if quota >= 0 {
        start as i32
    } else {
        -1
    }
}

fn can_complete_circuit_b(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let is_posible: i32 = gas.iter().zip(cost.iter()).map(|(&g, &c)| g - c).sum();

    println!("{is_posible}");

    let mut tank = 0;

    todo!();
}
