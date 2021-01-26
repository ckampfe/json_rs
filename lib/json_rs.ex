defmodule JsonRs do
  @moduledoc """
  """

  use Rustler, otp_app: :json_rs, crate: :json_rs

  def encode(term), do: encode(term, false)
  def encode(_term, _pretty), do: error()

  def encode!(term), do: encode!(term, false)

  def encode!(term, pretty) do
    case encode(term, pretty) do
      {:ok, term} -> term
      {:error, error} -> raise error
    end
  end

  def decode(_string), do: error()

  defp error() do
    :erlang.nif_error(:nif_not_loaded)
  end
end
