{-
Welcome to a Spago project!
You can edit this file as you like.
-}
{ name = "my-project"
, dependencies = [ "console", "effect", "psci-support", "node-fs", "generics-rep", "arraybuffer", "refs", "sequences" ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs" ]
}
