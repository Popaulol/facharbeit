import glob
import pathlib


def generate_single_include(f: open, filename: str, language: str = "rust"):
    if "tests" in filename:
        return
    f.write(r"\subsubsection*{" + filename.replace("_", r"\_") + "}\n")
    f.write(r"\inputminted[linenos=true, breaklines=true, breakautoindent=true]{" + language + "}{" + str(pathlib.Path(filename).absolute()) + "}\n")
    f.write(r"\newpage")
    f.write("\n\n")


if __name__ == '__main__':
    with open("code_include.txt", "w") as f:
        for file in glob.glob("src/**/*.rs", recursive=True):
            print(file)
            generate_single_include(f, file)
        generate_single_include(f, "Cargo.toml", "toml")
