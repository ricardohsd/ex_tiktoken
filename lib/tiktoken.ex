defmodule Tiktoken do
  @moduledoc """
  Documentation for `Tiktoken`.
  """

  @model_to_encoding %{
    "P50kBase" => Tiktoken.P50K,
    "P50kEdit" => Tiktoken.P50KEdit,
    "R50kBase" => Tiktoken.R50K,
    "Cl100kBase" => Tiktoken.CL100K,
    "Gpt2" => Tiktoken.P50K,
  }

  def get_tokenizer(model) do
    encoding_name = Tiktoken.Native.get_tokenizer(model)
    @model_to_encoding[encoding_name]
  end
end
