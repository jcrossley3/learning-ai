## Transformers Python examples

### Python virtual environment
```console
$ source pytorch/bin/activate
```

### Prerequisites
Some of the example use IPython.core.display.HTML to show graphs and tables
which require a web browser and using Jupyter Notbook seems to be the easiest
way to enables this.
Install jupyter:
```console
(pytorch) $ pip install jupyter
```

### Starting Jupyter notebook
Start a notbook in the current directory:
```console
(pytorch) $ jupyter notebook
```
This will open up a directory listing in a web browser. 

To create a new notebook, there is a `New` button on the right hand side and
it will allow you to specify the `Python3` kernel. Using that will open and
unnamed file which we can then rename.

### Keyboard shortcuts
The section contains Keyboard shortcuts in jupyter notbooks. One thing to
remember is that if you are using the vim chrome plugin that disable it as it
will otherwise interfer with the shortcuts in notebooks.

* CTRL+ENTER        Execute cell 
* A                 New cell Above the current one
* B                 New cell Below the current one
 
