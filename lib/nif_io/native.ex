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
  require Rustler

  @on_load :load_nif

  def load_nif do
    Rustler.load_nif("io")
  end

  defp err, do: throw :nif_not_loaded

  def open(_options), do: err
  def read_until(_resource, _byte), do: err

end
