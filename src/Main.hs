module Main (main) where

import Options.Applicative
  ( Parser,
    ParserInfo,
    argument,
    command,
    eitherReader,
    execParser,
    fullDesc,
    header,
    help,
    helper,
    info,
    metavar,
    progDesc,
    subparser,
    (<**>),
  )
import Text.Casing (camel, kebab, pascal, quietSnake, screamingSnake)

newtype Action = ConvertCase CaseType
  deriving (Show)

data CaseType = Pascal | Camel | Snake | ScreamingSnake | Kebab
  deriving (Read, Show)

parseCaseType :: String -> Either String CaseType
parseCaseType caseType
  | caseType `elem` ["p", "pascal"] = Right Pascal
  | caseType `elem` ["c", "camel"] = Right Camel
  | caseType `elem` ["s", "snake"] = Right Snake
  | caseType `elem` ["ss", "screaming-snake"] = Right ScreamingSnake
  | caseType `elem` ["k", "kebab"] = Right Kebab
  | otherwise = Left $ "Unknown case type: " ++ caseType

caseTypeParser :: Parser CaseType
caseTypeParser =
  argument
    (eitherReader parseCaseType)
    ( metavar "CASE" <> help "Case type to transform"
    )

actionParser :: Parser Action
actionParser =
  subparser $
    command
      "c"
      ( info (ConvertCase <$> caseTypeParser) (progDesc "Convert text to specified case")
      )

opts :: ParserInfo Action
opts =
  info
    (actionParser <**> helper)
    ( fullDesc
        <> progDesc "A small collection of utilities for use with Helix"
        <> header "hx-utils"
    )

transformText :: CaseType -> String -> String
transformText = \case
  Pascal -> pascal
  Camel -> camel
  Snake -> quietSnake
  ScreamingSnake -> screamingSnake
  Kebab -> kebab

main :: IO ()
main = do
  action <- execParser opts
  case action of
    ConvertCase caseType -> do
      input <- getContents
      putStr $ transformText caseType input
