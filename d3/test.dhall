let base = ./spago.dhall

in base // {
  sources = base.sources # [ "**/*.purs.spec" ],
  dependencies = base.dependencies # [ "spec", "spec-discovery" ]
}