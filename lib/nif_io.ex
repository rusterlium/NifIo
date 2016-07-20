defmodule NifIo do

  # TODO: Make this function more File.open
  def open(path) when is_binary(path) do
    options = %NifIo.Native.FileOpenOptions{path: path}
    case NifIo.Native.open(options) do
      {:error, err} -> {:error, err}
      res -> {:ok, NifIo.FileHandle.wrap_resource(res)}
    end
  end

end

