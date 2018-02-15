defmodule NifIo.Mixfile do
  use Mix.Project

  def project do
    [app: :nif_io,
     version: "0.1.0",
     elixir: "~> 1.3",
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     compilers: [:rustler] ++ Mix.compilers,
     rustler_crates: rustler_crates(),
     deps: deps()]
  end

  def application do
    [applications: [:logger]]
  end

  defp deps do
    [{:rustler, github: "hansihe/rustler", sparse: "rustler_mix"}]
  end

  defp rustler_crates do
    [io: [
      path: "native/io",
      mode: rustc_mode(Mix.env)
    ]]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug
end
