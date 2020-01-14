# Getty Hierarchy
This repository contains both a binary executable and a Python module written in Rust:  
* The executable can be used to parse one of the [Getty vocabularies](http://vocab.getty.edu/) NTriples dumps
into two hashmaps (read below for info on how these were built):  
  1. One holding the vocabulary hierarchy;  
  2. One being a `term_id: term_label` map.  
* The Python module is very simple and can be used to reconstruct the hierarchy of a term, given its id.  
  
The purpose of this project is to offer an easy and fast way to reconstruct the hierarchy of any term in Python, 
which can come in handy when this operation needs to be performed several times, without the need 
to call the Getty API every time.  
**NOTICE**: both the module and the executable have been tested on Linux only, against the AAT vocabulary only.

## Little disclaimer
Notice that we use term or node as synonyms. I am nowhere near an expert of Getty's vocabularies nor an expert of the 
Linked Open Data format and this project was born as a weekend project, since I needed to reconstruct the hierarchy of 
labels used in a dataset (calling the web API proved to be quite expensive for a large number of labels).
  
## The Python module
### Installing
This repository offers Python wheels for versions 3.6, 3.7 and 3.8 which were built with [Maturin](https://github.com/PyO3/maturin)
(you can find them in the `binaries` directory, at the root of this repository).
This means you can simply install the module by downloading the right wheel file for your Python version
and run:  

`pip install getty_hierarchy-1.0.0-cp3x-cp3xm-manylinux1_x86_64.whl`  

At the moment, the repository hosts Linux wheels only, but I'm planning to release for other platforms as well
(see [Building from source](#building-from-source) if you need
the package for another platform in the meantime).  
  
### Usage
Once installed, usage of the module is pretty straightforward.  
First, you need to import the `getty_hierarchy` module:
```python
import getty_hierarchy
```  
Then, you should construct the only class exposed by the API, that is the `Hierarchy` class.  
You can do this in two ways:  
by calling the base constructor
```python
hierarchy = getty_hierarchy.Hierarchy()
```
This will load into memory both the hierarchy and term-label pickled maps for the AAT vocabulary.  
If instead you wish to use your own pickled files, you can do that by calling the `Hierarchy.from_custom_pickled` static method
```python
hierarchy = getty_hierarchy.Hierarchy.from_custom_pickled(
        "./path_to_pickled_hierarchy.bin", 
        "./path_to_pickled_termmap.bin"
    )
```
The first argument is the path to the pickled hierarchy map, whereas the second is the path to 
the pickled term-label map: these can be generated with the 
binary executable offered in this repository (see [Using the executable](#using-the-executable)).  
Furthermore, you can pass an empty string as the second argument if you don't need the term-label map. 
  
Once the `Hierarchy` class is constructed, you can use two methods:  
the `get_parents` method will return the list of a term parents, sorted so that 
the closest parent comes first:

```python
# eg. node_id = 300053049
hierarchy.get_parents(node_id)

# Output
[300053043, 300229467, 300053003, 300053001, 300264090]
```  
If you feel the need, you can test the parents list correctness by visiting the [web API page](http://vocabsservices.getty.edu/AATService.asmx?op=AATGetParents)
and using id `300053049` as input.  
if a term has more than one parent (or if its parents do), the output will be as follows:
```python
hierarchy.get_parents(300073708)

# Output
[[300055980, 300055126, 300264086], [300389850, 300015646, 300264088]]
```
The *religions* term has two parents in the AAT hierarchy, whose ids are `300055980` and `300389850`; as you may notice,
the resulting list in the Python code is actually a list of lists:  
the preferred parent (more info in the [How the hierarchy is built](#how-the-hierarchy-is-built) section) comes 
first and the other parents come afterwards (with no specific order). This means that the first list in our 
example contains the "preferred" hierarchy (eg. what is listed [here](http://www.getty.edu/vow/AATFullDisplay?find=300073708&logic=AND&note=&english=N&prev_page=1&subjectid=300073708)
under *Hierarchical position*) and the second list holds what is listed [here](http://www.getty.edu/vow/AATFullDisplay?find=300073708&logic=AND&note=&english=N&prev_page=1&subjectid=300073708)
under *Additional parents*.  
This is a recursive behaviour, meaning that if in this example node `300055126`
had two parents, then we would have had a list of lists as the third element, instead of the *term_id* `300264086`.  
  
The second method you can use on the `Hierarchy` class is the `get_node_label` method, which you can use to retrieve 
any term preferred label (again, see [How the hierarchy is built](#How the hierarchy is built) for more info).  
It works pretty intuitively as follows:
```python
hierarchy.get_node_label(300053049)

# Output 
# dyeing
```
**Notice**: this will output an empty string in case you constructed the `Hierarchy` class with the 
`from_custom_pickled` static method and passed an empty string as the second argument. 

## [Building from source](#building-from-source)
In order to build from source you will need to install the Rust nightly toolchain: see [here](https://www.rust-lang.org/learn/get-started)
for how to install Rust and [here](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly)
for how to use the nightly toolchain.  
Once you correctly setup everything, build by simply running:  

`cargo build --release`  

you will find the executable in `target/release/getty_hierarchy_bin`.  
To build the Python module, I recommend using [Maturin](https://github.com/PyO3/maturin):  

`maturin build --release`  

you will find the Python wheels in `target/wheels/`.

## [Using the executable](#using-the-executable)
Download the `getty_hierarchy_bin` file you can find in the `binaries` directory at the root of this repository 
(or build it). You can list all the options by running:  

`./getty_hierarchy_bin --help`  

which should produce the following output:  
```
Getty hierarchy builder 1.0
Alessio Molinari <alessio.molinari@isti.cnr.it>

This program will parse a getty nt triples file into two maps: one will be a hierarchy map, the other will be a
term_id:label map (optional).
Finally, these maps will be bincoded (or pickled, if you're familiar with Python) into two file .bin which you will find
in your current working directory.
These files can be used in the Python module to quickly build the hierarchy or the term:label map.

USAGE:
    getty_hierarchy_bin [FLAGS] <FILE>

FLAGS:
    -h, --help           Prints help information
    -V, --version        Prints version information
        --no-term-map    If set, the program won't generate the term:label map

ARGS:
    <FILE>    Getty NT triples dump file
```  
### Usage example: generate the hierarchy and term:label maps
First, download one of the Getty NTriples zip, eg. the [AAT](http://vocab.getty.edu/dataset/aat/full.zip) one and
extract it. Then you can run:  
`./getty_hierarchy_bin AATOut_Full.nt`  
This will probably take a minute or two and will output the two pickled files `bincoded_hierarchy.bin` and
`bincoded_termmap.bin`.

## [How the hierarchy is built](#how-the-hierarchy-is-built)
The hierarchy map is built by parsing the NTriples dump available on the Getty website.
The parents of any term are retrieved using the http://vocab.getty.edu/ontology#broader property.  
If any term has more than one parent, the preferred parent (retrieved via the http://vocab.getty.edu/ontology#broaderPreferred 
property) is listed before the others.  
The `term:label` map is instead built by retrieving the http://vocab.getty.edu/ontology#term
and the http://vocab.getty.edu/ontology#prefLabelGVP properties: however, the *term id* 
which should be used to retrieve the label is the same used to retrieve a term parents in the hierarchy map
(instead of using the http://vocab.getty.edu/aat/term/*id* term id).  

### More technical details
As a consequence of what we said in the previous section, we need to allocate a temporary HashMap to build 
the `term:label` map. This is better explained with an example: when we build the `hierarchy` map
we use the *term_id* retrieved with the http://vocab.getty.edu/ontology#broader property as a key.
For any *term*, this *term_id* is not the same we get from the http://vocab.getty.edu/ontology#term property 
and, as a consequence, if we wanted to retrieve the parent labels of a term, we'd have to use different ids wrt. 
what we have in the returned list. For instance, the id of the *term* [dyeing](http://vocab.getty.edu/aat/300053049) is 
`300053049`; however, its label has id `1000053049` (see [here](http://vocab.getty.edu/aat/term/1000053049-en)). 
In order to allow us to map the *term_id* we use in the hierarchy to its label value as a string, we need to first 
retrieve the (let's call it) *term_label_id*, map it to its string value and then remap that string value to the above
mentioned *term_id*. As such, building the `term:label` map has a quite consistent overhead compared to simply 
building the `hierarchy` map and that is why it is possible to entirely skip it, both in the executable and in the
Python module. If you have any suggestion on how to improve this, feel free to open an issue :)


## Credits
This project was inspired and helped by the already existing repository from mdlincoln, available at 
https://github.com/mdlincoln/getty_vocab (those scripts in ruby were pretty helpful to understand how to approach
this in the first place). Also, take a look at his blog post here https://matthewlincoln.net/2014/02/21/hierarchies-of-the-getty.html

Of course, this whole project was made possible by the dumps made available by the Getty project at http://vocab.getty.edu
