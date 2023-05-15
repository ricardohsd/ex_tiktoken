defmodule ExTiktoken do
  @moduledoc """
  Documentation for `ExTiktoken`.
  """

  @model_to_encoding %{
    "P50kBase" => ExTiktoken.P50K,
    "P50kEdit" => ExTiktoken.P50KEdit,
    "R50kBase" => ExTiktoken.R50K,
    "Cl100kBase" => ExTiktoken.CL100K,
    "Gpt2" => ExTiktoken.P50K,
  }

  def get_tokenizer(model) do
    encoding_name = ExTiktoken.Native.get_tokenizer(model)
    @model_to_encoding[encoding_name]
  end
end
