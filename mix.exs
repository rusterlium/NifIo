defmodule NifIo.Mixfile do
  use Mix.Project

  def project do
    [
      app: :nif_io,
      version: "0.1.0",
      elixir: "~> 1.10",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.22-rc"}
    ]
  end

  defp rustler_crates do
    [
      io: [
        path: "native/io",
        mode: rustc_mode(Mix.env())
      ]
    ]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug

  defp aliases do
    [
      fmt: [
        "format",
        "cmd cargo fmt --manifest-path native/io/Cargo.toml"
      ]
    ]
  end
end
