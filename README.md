# シンプルフォーム社 - 共通ライブラリ

## 環境構築

### netrc の設定

以下、pip インストールのための事前設定として、.netrc の設定が必要となります。
設定ファイル`~/.netrc`を作成し、以下のような認証情報を入力してください。

```
[~/.netrcの例]

machine github.com
login <GITHUB_ACCESS_TOKEN>
password x-oauth-basic
```

## 自動テストの実行方法

### SSO ログイン

以下のコマンドを実行して stg 環境にログインします。

```sh
$ aws sso login --profile simplecheck-stg
```

ログインしたら、以下のコマンドを実行して一時認証情報を環境変数に格納します。

```sh
$ eval "$(aws2-wrap --profile simplecheck-stg --export)"
```

### テスト実行

```sh
$ SECRET_NAME=simplecheck/local poetry run poe test
```

### stg 環境の DB への接続が必要なテスト

一部、stg 環境の DB に接続しなければ実行できないテストがあります (本来は stg 環境の DB に接続する必要なく完結すべきなので、今後廃止予定です)。当該テストを実行したい場合は以下のコマンドを実行します。

```sh
$ SECRET_NAME=simplecheck/remote/stg poetry run python -m pytest --sf_db
```

## DB のマイグレーション実行方法

DB のテーブル定義の変更は以下の手順で実施します。

1. モデルファイルを修正する

   - `simplecheck/core/simplecheck/models` ディレクトリ内に SQLAlchemy のモデルファイルが格納されています。当該ディレクトリ内でモデルの定義を追加／編集します。

1. 以下のコマンドを実行して、ローカルの Docker で起動した MySQL に既存のマイグレーションを流す (事前に上に記載の SSO ログインの手順を踏んでください)

   ```sh
   $ docker compose run --rm core poetry run poe db migrate
   ```

1. 以下のコマンドを実行して、新しいマイグレーションファイルを生成する

   ```sh
   $ docker compose run --rm core poetry run poe db new_revision -m "some awesome comment"
   ```

1. `simplecheck/core/simplecheck/migrations/versions` ディレクトリ内に生成されたマイグレーションファイルを確認し、想定通りのマイグレーションになっていることを確認する

   - 以下のケースではマイグレーションファイルを手で書き換える必要があるので、ご注意ください
     - テーブル名変更、カラム名変更 (自動生成されたマイグレーションファイルでは削除 => 新規作成になるため)
     - Enum 型のカラムの定義変更 (自動生成されたマイグレーションファイルでは Enum 型のカラム定義変更は検知されないため)

1. 再度以下のコマンドを実行して、ローカルの Docker で起動した MySQL に新しく生成されたマイグレーションを流す。処理完了後、DB のテーブル定義が想定通り変わっていることを確認する

   ```sh
   $ docker compose run --rm core poetry run poe db migrate
   ```

1. 修正内容をコミットして PR を作成する

> [!NOTE]
> 各環境へのマイグレーションは GitHub Actions (GHA) で実行されるため、手動でのマイグレーションは不要です。
>
> - prod: リリースによりマイグレーションされます。
> - stg: main へのマージでマイグレーションされます。
> - itg: SimpleCheck をデプロイする GHA の実行によりマイグレーションされます。
>   - itg1: [Deploy SimpleCheck to itg1](https://github.com/simple-form/simpleform/actions/workflows/deploy_simplecheck_itg1.yml)
>   - itg2: [Deploy SimpleCheck to itg2](https://github.com/simple-form/simpleform/actions/workflows/deploy_simplecheck_itg2.yml)
>
> 手動でのマイグレーションが必要な場合は次のように環境名を指定して実行します (ただし、原則 `prod` へは実行しない)。
>
> ```sh
> $ docker compose run --rm core poetry run poe db migrate --env {stg|itg1|itg2}
> ```

### その他の主な使い方

1. DB の状態とモデルの状態との差分を表示

   ```sh
   $ docker compose run --rm core poetry run poe db diff
   ```

1. 現在の revision を表示

   ```sh
   $ docker compose run --rm core poetry run poe db current
   ```

1. ロールバック

   ```sh
   $ docker compose run --rm core poetry run poe db rollback
   ```

その他のオプション、コマンドの詳細は [simplecheck/core/simplecheck/migrations/scripts/cmd.py](https://github.com/simple-form/simpleform/blob/main/simplecheck/core/simplecheck/migrations/scripts/cmd.py) を参照してください。

## DB のリセット

1. docker コンテナを停止して volume を破棄します。

   ```sh
   docker compose down --volumes --remove-orphans
   ```
# SimpleAIHackason-Cteam
