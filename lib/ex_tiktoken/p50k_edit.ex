defmodule ExTiktoken.P50KEdit do
  @behaviour ExTiktoken.Encoding

  @impl ExTiktoken.Encoding
  def encode_ordinary(text) do
    ExTiktoken.Native.p50k_edit_encode_ordinary(text)
  end

  @impl ExTiktoken.Encoding
  def encode(text, allowed_special \\ []) do
    ExTiktoken.Native.p50k_edit_encode(text, allowed_special)
  end

  @impl ExTiktoken.Encoding
  def encode_with_special_tokens(text) do
    ExTiktoken.Native.p50k_edit_encode_with_special_tokens(text)
  end

  @impl ExTiktoken.Encoding
  def decode(ids) do
    ExTiktoken.Native.p50k_edit_decode(ids)
  end
end
