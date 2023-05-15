defmodule ExTiktoken.Native do
  @moduledoc false

  version = Mix.Project.config()[:version]

  use RustlerPrecompiled,
    otp_app: :ex_tiktoken,
    crate: :ex_tiktoken,
    base_url: "https://github.com/ricardohsd/ex_tiktoken/releases/download/v#{version}",
    force_build: System.get_env("RUSTLER_PRECOMPILATION_BUILD") in ["1", "true"],
    version: version,
    nif_versions: ["2.16", "2.15"],
    targets: [
      "arm-unknown-linux-gnueabihf",
      "aarch64-unknown-linux-gnu",
      "aarch64-apple-darwin",
      "riscv64gc-unknown-linux-gnu",
      "x86_64-apple-darwin",
      "x86_64-unknown-linux-gnu",
      "x86_64-pc-windows-gnu",
      "x86_64-pc-windows-msvc"
    ]

  def get_tokenizer(_model), do: err()

  def p50k_encode_ordinary(_input), do: err()
  def p50k_encode(_input, _allowed_special), do: err()
  def p50k_encode_with_special_tokens(_input), do: err()
  def p50k_decode(_ids), do: err()

  def p50k_edit_encode_ordinary(_input), do: err()
  def p50k_edit_encode(_input, _allowed_special), do: err()
  def p50k_edit_encode_with_special_tokens(_input), do: err()
  def p50k_edit_decode(_ids), do: err()

  def r50k_encode_ordinary(_input), do: err()
  def r50k_encode(_input, _allowed_special), do: err()
  def r50k_encode_with_special_tokens(_input), do: err()
  def r50k_decode(_ids), do: err()

  def cl100k_encode_ordinary(_input), do: err()
  def cl100k_encode(_input, _allowed_special), do: err()
  def cl100k_encode_with_special_tokens(_input), do: err()
  def cl100k_decode(_ids), do: err()

  defp err, do: :erlang.nif_error(:nif_not_loaded)
end
