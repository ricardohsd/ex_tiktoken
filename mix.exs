defmodule ExTiktoken.MixProject do
  use Mix.Project

  @version "0.1.1"
  @url "https://github.com/ricardohsd/ex_tiktoken"

  def project do
    [
      app: :ex_tiktoken,
      version: @version,
      elixir: "~> 1.13",
      description: "Elixir bindings for Tiktoken tokenizer",
      source_url: @url,
      homepage_url: @url,
      start_permanent: Mix.env() == :prod,
      package: package(),
      deps: deps(),
      licenses: licenses(),
      aliases: aliases()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:ex_doc, "~> 0.27", only: :dev, runtime: false},
      {:rustler, "~> 0.27.0"},
      {:rustler_precompiled, "~> 0.6"}
    ]
  end

  defp licenses, do: ~w(MIT)

  defp package do
    [
      files: ~w(lib priv .formatter.exs mix.exs README* LICENSE* native checksum-*.exs),
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/ricardohsd/ex_tiktoken"}
    ]
  end

  defp aliases do
    [
      fmt: [
        "format",
        "cmd cargo fmt --manifest-path native/ex_tiktoken/Cargo.toml"
      ]
    ]
  end
end
