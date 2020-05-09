# Questions

## Partie 1
<p>1.1  En rust les references permettent d'acces aux valuers </p>
<p>1.2 let foo: i32 = 117; &nbsp;&nbsp;&nbsp;&nbsp; let bar = 117i32;</p>
<p>1.3 Rust est compilé nativement</p>
<p>1.4</p>
<p>1.5 Un processus permet la génération ou l'interaction avec d'autres processus (doc rust)</p>

## Partie 2
<p>2.1 Pour compiler puis executer un programme en rust (dans notre cas) il suffit de faire </p>

```
cargo run
```

<p>Pour executer un test en rust il faut avoir</p>

```
#[cfg(test)]
mod tests {
    ...
}
```

<p>2.4 Rust nous force à récupérer le statut pour les collections en attendant la fin du processus enfant</p>
<p>2.5 Le programme est en état wait du processus enfant </p>
<p>2.7</p>
<p>2.10 Un processus id est l'id d'un processus attribué à l'OS</p>