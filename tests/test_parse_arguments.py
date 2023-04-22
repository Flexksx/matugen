from argparse import Namespace
import pytest

from matugen.util import parse_arguments, InvalidFileExtension

VALID_WALLPAPER_PATH = "example/wallpaper.jpg"
INVALID_WALLPAPER_PATH = "invalid_path"
NONEXISTENT_WALLPAPER_PATH = "nonexistent.jpg"
INVALID_WALLPAPER_EXTENSION = "not_an_image.txt"

VALID_CONFIG_PATH = "example/config.ini"
INVALID_CONFIG_PATH = "invalid_path"
INVALID_CONFIG_EXTENSTION = "invalid.extension"
NONEXISTENT_CONFIG_PATH = "nonexistent.ini"


class TestParseArguments:
    # # Tests that parse_arguments function correctly parses a valid wallpaper path.
    def test_parse_arguments_with_valid_wallpaper_path(self, mocker):
        # Arrange
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=VALID_WALLPAPER_PATH, config=VALID_CONFIG_PATH
            ),
        )

        # Act
        args = parse_arguments()

        # Assert
        assert args.wallpaper == VALID_WALLPAPER_PATH

    # Tests that parse_arguments function correctly parses the light mode flag.
    def test_parse_arguments_with_light_mode_flag(self, mocker):
        # Arrange
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=VALID_WALLPAPER_PATH, config=VALID_CONFIG_PATH, lightmode=True
            ),
        )

        # Act
        args = parse_arguments()

        # Assert
        assert args.lightmode == True

    # Tests that parse_arguments function raises an error when provided with an invalid wallpaper path.
    def test_parse_arguments_with_invalid_wallpaper_path(self, mocker):
        # Create a mock for the argparse.ArgumentParser.parse_args() method
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=INVALID_WALLPAPER_PATH, config=VALID_CONFIG_PATH
            ),
        )

        # Act and Assert
        with pytest.raises(FileNotFoundError):
            parse_arguments()

    # Tests that parse_arguments function raises an error when provided with a wallpaper path that does not exist.
    def test_parse_arguments_with_nonexistent_wallpaper_path(self, mocker):
        """
        Tests that parse_arguments function raises an error when provided with a wallpaper path that does not exist.
        """
        # Create a mock for the argparse.ArgumentParser.parse_args() method
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=NONEXISTENT_WALLPAPER_PATH, config=VALID_CONFIG_PATH
            ),
        )

        with pytest.raises(FileNotFoundError):
            parse_arguments()

    # Tests that parse_arguments function raises an error when provided with an invalid config path.
    def test_parse_arguments_with_invalid_config_path(self, mocker):
        """
        Tests that parse_arguments function raises an error when provided with an invalid config path.
        """
        # Create a mock for the argparse.ArgumentParser.parse_args() method
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=VALID_WALLPAPER_PATH, config=INVALID_CONFIG_PATH
            ),
        )

        with pytest.raises(FileNotFoundError):
            parse_arguments()

    # Tests that parse_arguments function raises an error when provided with a nonexistent config path.
    def test_parse_arguments_with_nonexistent_config_path(self, mocker):
        """
        Tests that parse_arguments function raises an error when provided with an nonexistent config path.
        """
        # Create a mock for the argparse.ArgumentParser.parse_args() method
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=VALID_WALLPAPER_PATH, config=NONEXISTENT_CONFIG_PATH
            ),
        )

        with pytest.raises(FileNotFoundError):
            parse_arguments()

    # Tests that parse_arguments function raises an error when provided with a nonexistent config path.
    def test_parse_arguments_with_invalid_config_extension(self, mocker):
        """
        Tests that parse_arguments function raises an error when provided with an invalid config extension.
        """
        # Create a mock for the argparse.ArgumentParser.parse_args() method
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=VALID_WALLPAPER_PATH, config=INVALID_CONFIG_EXTENSTION
            ),
        )

        with pytest.raises(InvalidFileExtension):
            parse_arguments()

    # Tests that parse_arguments function raises an error when provided with a wallpaper path that is not an image file.
    def test_parse_arguments_with_invalid_wallpaper_extension(self, mocker):
        """
        Tests that parse_arguments function raises an error when provided with a wallpaper path that is not an image file.
        """
        # Create a mock for the argparse.ArgumentParser.parse_args() method
        mocker.patch(
            "argparse.ArgumentParser.parse_args",
            return_value=Namespace(
                wallpaper=INVALID_WALLPAPER_EXTENSION, config=VALID_CONFIG_PATH
            ),
        )

        with pytest.raises(InvalidFileExtension):
            parse_arguments()
