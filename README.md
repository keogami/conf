# Conf
I don't know where my configs are, but conf does. Just tell it to alias the path you want it to remember:

```bash
conf alias hypr ~/.config/hypr
```

Then ask it when you wanna tweak your config.
```bash
conf edit hypr
```

## Okay, but I ain't gonna remember what alias I set for my config
Worry not, you can use piping with our beloved `fzf` to get fuzzy find your configs
```bash
conf list | fzf | xargs conf edit
```

## Okay, but I ain't gonna remember that command anyway
Just set an alias in your shell, and you are good to go :3

## Okay, but I ain't gonna remember that alias anyway
How about you shut up?

## Fine, how do I install
You can install using cargo:
```bash
cargo install --profile=release --git https://github.com/keogami/conf
```

---

Okay, byw
