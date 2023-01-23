# adventofcode

I've been reading The Book to learn rust, and came across [this youtube video](https://www.youtube.com/watch?v=U16RnpV48KQ). I decided to tackle these problems as a platform to learn rust.

## Important things I learned 
### 2022
#### day 4
- regex crate
- lazy_static
- ```git rm --cached -r '*/*/target*'``` worked to delete all the target files from this repo, but it looks like ```'**/target*'``` would have been the idiomatic way to do this.
#### day 5
- resize a vec like
```
        self.stacks.clear();
        self.stacks.resize(num_crates, Default::default());
```