# NOCE

Noce is a terminal code editor, it attempts to offer a flexible and customizable envoirment while being easy to use (at least more than neovim).
As now the project is totally unstable, it does not works in many fields and actually supports only navigation of the filesystem and basic editing of the opened files:

![pwsh-in-noce-2024-06-08-21-01-05](https://github.com/nunzioono/noce/assets/36959525/83772c72-c4bd-431f-a1cd-06fa2f9bf8ed)

# CONTRIBUTIONS


Contributions are highly encouraged, the road to go would likely include the following features:

- [ ] improving the editing:
   * [x] For now insertion, deletion and saving are implementer.
   * [ ] Need to debug, implement and test selection
   * [ ] Need to implement cut, Copy and past functionalities
- [ ] making a better terminal interaction.
- [ ] include a treesitter like system to implement in future syntax highlighting and LSP support for autocompletion and other interesting features.
- [ ] making a customizable ui (currently based on ratatui and crossterm).

