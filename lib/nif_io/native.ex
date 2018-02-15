defmodule NifIo.Native.FileOpenOptions do
  defstruct [
    path: nil,
    read: true,
    write: true,
    append: false,
    truncate: false,
    create: true,
    create_new: false,
  ]
end

defmodule NifIo.Native do
  use Rustler, otp_app: :nif_io, crate: :io

  def open(_options), do: error()
  def read_until(_resource, _byte), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)
end
