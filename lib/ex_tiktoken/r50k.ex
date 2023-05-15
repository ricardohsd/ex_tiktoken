defmodule ExTiktoken.R50K do
  @behaviour ExTiktoken.Encoding

  @impl ExTiktoken.Encoding
  def encode_ordinary(text) do
    ExTiktoken.Native.r50k_encode_ordinary(text)
  end

  @impl ExTiktoken.Encoding
  def encode(text, allowed_special \\ []) do
    ExTiktoken.Native.r50k_encode(text, allowed_special)
  end

  @impl ExTiktoken.Encoding
  def encode_with_special_tokens(text) do
    ExTiktoken.Native.r50k_encode_with_special_tokens(text)
  end

  @impl ExTiktoken.Encoding
  def decode(ids) do
    ExTiktoken.Native.r50k_decode(ids)
  end
end
