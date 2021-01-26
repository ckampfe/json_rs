defmodule JsonRs do
  @moduledoc """
  """

  use Rustler, otp_app: :json_rs, crate: :json_rs

  def encode(_term), do: error()
  def decode(_string), do: error()

  defp error() do
    :erlang.nif_error(:nif_not_loaded)
  end
end
