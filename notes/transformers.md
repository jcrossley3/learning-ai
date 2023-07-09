## Transformers
Before the transformers architecture was developed, which was done by Google in
2017. Recursive Neural Networks (RNN) had been used up until this point but
transformers outperformed in terms of translation quality and training cost.

### Timeline
2017 Transformers
2018 ULMFit
2018 GPT
2018 BERT
2019 GPT-2
2019 RoBERTa
2019 DistilBERT
2020 GPT-3
2020 DeBERTa
2020 T5
2021 GPT-Neo
2021 GPT-J


### Generative Pretrained Transformer (GPT)

### Bidirectional Encoder Representation from Transformers (BERT)

### T5


### Scaled dot product attention
In standard attention uses 3 martixes, a query matrix, a key matrix, and a value
matrix. The query and key matrixes contain wieghts, and the value matrix
contains an embedding of the value.

We start with our tokenized input, the embedded vector(s) that represents some
text, to in this example we assume that the input has already been through
BERT's preprocessing.
We can call this vector X, and lets say the dimension of this is 4 and that we
have two tokens:
```
          +--+--+--+--+
token 1   |  |  |  |  |
          +--+--+--+--+
token 2   |  |  |  |  |
          +--+--+--+--+
```

So we will have the folling matrix multilications:
```
       X                 W_Q              Q
  +--+--+--+--+      +--+--+--+
  |  |  |  |  |      |  |  |  |       +--+--+--+
  +--+--+--+--+      +--+--+--+    =  |  |  |  |     
  |  |  |  |  |      |  |  |  |       +--+--+--+
  +--+--+--+--+      +--+--+--+       |  |  |  |
                     |  |  |  |       +--+--+--+
                     +--+--+--+
                     |  |  |  |
                     +--+--+--+

       X                 W_K              K
  +--+--+--+--+      +--+--+--+
  |  |  |  |  |      |  |  |  |       +--+--+--+
  +--+--+--+--+      +--+--+--+    =  |  |  |  |     
  |  |  |  |  |      |  |  |  |       +--+--+--+
  +--+--+--+--+      +--+--+--+       |  |  |  |
                     |  |  |  |       +--+--+--+
                     +--+--+--+
                     |  |  |  |
                     +--+--+--+

       X                 W_V              V
  +--+--+--+--+      +--+--+--+
  |  |  |  |  |      |  |  |  |       +--+--+--+
  +--+--+--+--+      +--+--+--+    =  |  |  |  |     
  |  |  |  |  |      |  |  |  |       +--+--+--+
  +--+--+--+--+      +--+--+--+       |  |  |  |
                     |  |  |  |       +--+--+--+
                     +--+--+--+
                     |  |  |  |
                     +--+--+--+
```
Notice that we are takeing the input matrix X and doing 3 linear transformations
of the same vector. So we will have one result for multplying it with the query
vector, one for the key vector, and one for the value vector. This will produce
3 new vector spaces called the query space, the key space, and the value
space.

The Query vector is information that we are looking for, like the reason for
saying something when we speak. So since we multiply X with W_Q to get Q, the
Q vector has a context of sort which in the case of Query is the information
about the token and how it relates to the overall goal of the sentence.

The Key vectors is the relevance of the word(s), represented by X to the query. 
So the query, is to the query. Again since we mulitply X with W_K to get K, K
also has a context which is the relevance of X to the query.

The Value vector is just a compressed version of the embedded value. This is
called a context less version of the token.

Scaled dot product attention:
```
scaleced_dot_product(Q, K, V) = softmax((Q x Kᵗ)/√embedding_dim) x V
```
We use the softmax so that all the values add up to 1 but still keep the
individual difference (not sure about the terminology here).

```

     Q               Kᵗ         Attention matrix
  
  +--+--+--+       +--+--+     +--+--+               Higher value for attention
  |  |  |  |       |  |  |     |  |  |
  +--+--+--+   x   +--+--+ =   +--+--+
  |  |  |  |       |  |  |     |  |  |
  +--+--+--+       +--+--+     +--+--+
                   |  |  |
                   +--+--+

  +--+--+        +--+--+                        Only done to make the values
  |  |  |        |  |  |                        smaller and faster to compute
  +--+--+ / √3 = +--+--+                        without loosing any information.
  |  |  |        |  |  |
  +--+--+        +--+--+

                 +--+--+
                 |  |  |
        softmax( +--+--+ )
                 |  |  |
                 +--+--+
                                V        
                 +--+--+    +--+--+--+     +--+--+--+
                 |  |  |    |  |  |  |     |  |  |  |
                 +--+--+  X +--+--+--+  =  +--+--+--+
                 |  |  |    |  |  |  |     |  |  |  |
                 +--+--+    +--+--+--+     +--+--+--+

```
Input: "I like icecream"
Visualize Q vector space as vectors in two dimensions and we have three vectors,
one for "I", one for "like", and one for "icecream".  And we also have a vector
space for K with 3 vectors. When we calculate Q x Kᵗ we are getting a new square
matrix, and the values in this matrix contain the attention scores.
