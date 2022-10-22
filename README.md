
# Deep learning in Rust

This project is for me to learn Rust

*TO DO*
* fix conv2D
* multithreading


*Example Usage*
```rust
// Define NN
fn _simple(dim0: usize, dim1: usize) -> Box<Node> {
    let x_placeholder = Tensor::zeros(&vec![dim0, dim1]);
	let mut x_node = leaf(x_placeholder, false);
    x_node.op = "input";
    let l1 = linear(x_node, 100);
    let s1 = sigmoid(l1);
    let d1 = dropout(s1, 0.1);
    let l2 = linear(d1, 10);
    log_softmax(l2)
}

let mut out = _simple(1, 784);

// Optimizer
let mut opt = Adam {
    t: 0.0, 
    alpha: lr, 
    prev_m1s: Vec::new(),
    prev_m2s: Vec::new(),
};
opt.init_prevs(out.as_ref());

// Save best params
let mut best_epoch = 0;
let mut best_epoch_loss = 999999.9;
let mut best_params: Vec<Tensor> = Vec::new();
init_best(best_params.as_mut(), out.as_ref());

// Batch gradients
let mut batch_grads: Vec<Tensor> = Vec::new();
init_batch_grads(&mut batch_grads, &out);

for epoch in 0..epochs {
    for i in 0..x_train.len() {
        let x = &x_train[i]; 
        let y = &y_train[i];
        out = forward(out, x);
        nll_loss(&mut out, y);
        out = backward(out);
        // batch() handles minibatching, optimizer step, learning rate decay
        batch(i, &mut batch_grads, &mut out, &mut opt, batch_size, dataset_size, lr);
    }

    let mut total_loss = 0.0;
    for t in 0..x_test.len() {
        let x = &x_test[t]; 
        let y = &y_test[t]; 
        out = forward(out, &x);
        let loss = nll_loss(&mut out, y);
        total_loss += loss;
    }
    
    if total_loss < best_epoch_loss {
        best_epoch_loss = total_loss;
        best_epoch = epoch;
        save_best(best_params.as_mut(), out.as_mut(), 0);
    }
}
```
