defmodule NifIo.Native do
  require Rustler

  @on_load :load_nif

  def load_nif do
    Rustler.load_nif("io")
  end

  defp err, do: throw :nif_not_loaded

  def open_read_file(_path), do: err
  def read_line(_resource), do: err

end
