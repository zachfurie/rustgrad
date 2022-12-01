
# Deep learning in Rust

This project was a way for me to learn Rust and get more familiar with the internals of neural networks.


*Example Usage*
```rust
// Define NN
fn _simple(shape: &Vec<usize>) -> Box<Node> {
    let x_node = input(shape);
    let l1 = linear(x_node, 100, "xavier");
    let s1 = sigmoid(l1);
    let l2 = linear(s1, 10, "xavier");
    log_softmax(l2)
}

// Get shape of tensors in batched training set, pass to NN
let mut out = _simple(&x_train[0].shape); 

// Optimizer
let mut opt = Adam {
    t: 0.0, 
    alpha: 0.01, 
    prev_m1s: Vec::new(),
    prev_m2s: Vec::new(),
};
opt.init_prevs(&out); // Initialize optimizer

// Training on batched inputs
for epoch in 0..epochs {
    let mut total_loss = 0.0;
    for i in 0..x_train.len() {
        let x = &x_train[i]; 
        let y = &y_train[i];
        out = forward(out, x);
        total_loss += nll_loss(&mut out, y, false);
        out = backward(out);
        opt.step(&mut out);
}
```
