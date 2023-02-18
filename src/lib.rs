#[derive(Debug)]
pub struct Attributes {
    description: String,
    pkgs: String,
}

pub const TEMPLATE: &str = r#"
{
  description = <description>;
  inputs.nixpkgs.url = "github:nixos/nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "aarch64-darwin" "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ <packages> ];
        };
      });
}

"#;

impl Attributes {
    pub fn new(description: &str, pkgs: &str) -> Self {
        let description = String::from(description);
        let pkgs = String::from(pkgs);
        Attributes { description, pkgs }
    }

    pub fn generate(&self) -> String {
      format!("{}",TEMPLATE
                    .replace("<description>", self.description.as_str())
                    .replace("<packages>", self.pkgs.as_str()))
    }
}
