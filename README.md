# Datadog Profiler Demo

このリポジトリは、Datadog Continuous Profilerの検証を目的としたデモ用プロジェクトです。  
Goアプリケーションを使用して、プロファイリングの効果を可視化し、パフォーマンス改善の手法を学習できます。

## 概要

このデモでは、意図的に異なるパフォーマンス特性を持つ2つのGoアプリケーション（v1, v2）を用意し、Datadog Profilerを使ってその差異を可視化します。

### アプリケーションの特徴

#### app-v1（非効率版）
- **バブルソート**を使用したソートアルゴリズム
- 時間計算量: O(n²)
- CPUを多く消費する処理

#### app-v2（効率版）
- **標準ライブラリのソート**を使用
- 時間計算量: O(n log n)
- CPU使用量が大幅に改善

### 検証内容

1. **CPUプロファイル比較**: v1とv2のCPU使用パターンの違い
2. **メモリプロファイル比較**: ヒープ使用量の差異
3. **実行時間の比較**: 同じ処理での処理時間の違い
4. **負荷テスト**: k6を使った継続的な負荷による長期間の観測

## アーキテクチャ

```
┌─────────────────┐    ┌─────────────────┐
│   k6 Load Test  │───▶│  Go Apps (v1/v2)│
└─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │  Datadog Agent  │
                       └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │  Datadog Cloud  │
                       └─────────────────┘
```

## 必要な環境

- **Kubernetes クラスタ**（minikube推奨）
- **Datadog アカウント**とAPIキー
- **kubectl**
- **Docker**（イメージビルド用）

## 環境構築

### 1. minikubeの起動

```bash
# minikubeを起動
minikube start

# Dockerデーモンをminikube内に向ける（ローカルビルド用）
eval $(minikube docker-env)
```

### 2. Datadog APIキーの設定

```bash
# Datadog APIキーをSecretとして作成
kubectl create secret generic datadog-secret \
  --from-literal=api-key=<YOUR_DATADOG_API_KEY>
```

### 3. Datadog Operatorのインストール

```bash
# Datadog Operatorをインストール
kubectl apply -f https://github.com/DataDog/datadog-operator/releases/latest/download/datadog-operator.yaml

# DatadogAgentリソースをデプロイ
kubectl apply -f datadog-agent.yaml
```

### 4. Goアプリケーションのビルド（オプション）

既存のイメージを使用する場合はスキップ可能です。

```bash
# app-v1のビルド
cd app-v1
docker build -t app-v1:latest .

# app-v2のビルド
cd ../app-v2
docker build -t app-v2:latest .
```

## デプロイ手順

### 1. 設定データの作成

```bash
# input.txtをConfigMapとして作成
kubectl apply -f deployments/configmap.yaml
```

### 2. Goアプリケーションのデプロイ

```bash
# app-v1をデプロイ
kubectl apply -f deployments/app-v1.yaml

# app-v2をデプロイ
kubectl apply -f deployments/app-v2.yaml
```

### 3. デプロイ状況の確認

```bash
# Pod状況確認
kubectl get pods

# Service確認
kubectl get services

# ログ確認
kubectl logs -l app=app-v1
kubectl logs -l app=app-v2
```

## 負荷テストの実行

### 1. k6テストスクリプトの準備

```bash
# k6用ConfigMapを作成
kubectl apply -f deployments/k6-configmap.yaml
```

### 2. 負荷テストの実行

```bash
# k6 Jobを実行
kubectl apply -f deployments/k6.yaml
```

### 3. テスト状況の確認

```bash
# Job状況確認
kubectl get jobs

# ログ確認
kubectl logs job/k6-load-v1
kubectl logs job/k6-load-v2
```

## アプリケーションへのアクセス

### minikube service経由

```bash
# app-v1へのアクセスURL取得
minikube service app-service-v1 --url

# app-v2へのアクセスURL取得
minikube service app-service-v2 --url
```

### ポートフォワード経由

```bash
# app-v1（ポート8081でアクセス）
kubectl port-forward service/app-service-v1 8081:8080

# app-v2（ポート8082でアクセス）
kubectl port-forward service/app-service-v2 8082:8080
```

アクセス先:
- app-v1: http://localhost:8081
- app-v2: http://localhost:8082

## Datadog UIでの確認

### 1. APM（Application Performance Monitoring）

[Datadog APM](https://app.datadoghq.com/apm/services) にアクセスし、以下を確認：

- サービス一覧で `app` サービスを選択
- v1.0.0とv2.0.0のバージョン別パフォーマンス比較
- レスポンス時間、エラー率、スループットの違い

### 2. Continuous Profiler

[Datadog Profiler](https://app.datadoghq.com/profiling) にアクセスし、以下を確認：

- **CPUプロファイル**: `count` 関数の実行時間比較
- **ヒーププロファイル**: メモリ使用量の違い
- **フレームグラフ**: 関数呼び出しの可視化

### 3. 比較機能

- バージョン間の比較機能を使用
- 差分表示でパフォーマンス改善効果を定量化

## 期待される結果

### app-v1（バブルソート版）
- CPUプロファイルで `count` 関数が大きな割合を占める
- ネストしたループによる高いCPU使用率
- 処理時間が長い

### app-v2（標準ソート版）
- CPUプロファイルで `sort.Ints` の使用が確認できる
- 効率的なアルゴリズムによる低いCPU使用率
- 処理時間が大幅に短縮

## トラブルシューティング

### Pod起動エラー

```bash
# Pod詳細確認
kubectl describe pod <pod-name>

# ログ確認
kubectl logs <pod-name>
```

### Datadog Agent接続エラー

```bash
# DatadogAgent状態確認
kubectl get datadogagent
kubectl describe datadogagent datadog

# Agent Pod確認
kubectl get pods -l app.kubernetes.io/name=datadog-agent
```

### Service接続エラー

```bash
# Service詳細確認
kubectl describe service app-service-v1
kubectl describe service app-service-v2

# エンドポイント確認
kubectl get endpoints
```

### k6テストエラー

```bash
# ConfigMap確認
kubectl describe configmap k6-script

# Job再実行
kubectl delete job k6-load-v1 k6-load-v2
kubectl apply -f deployments/k6.yaml
```

## クリーンアップ

```bash
# すべてのリソースを削除
kubectl delete -f deployments/k6.yaml
kubectl delete -f deployments/app-v1.yaml
kubectl delete -f deployments/app-v2.yaml
kubectl delete -f deployments/k6-configmap.yaml
kubectl delete -f deployments/configmap.yaml
kubectl delete -f datadog-agent.yaml

# Secret削除
kubectl delete secret datadog-secret

# minikube停止
minikube stop
```

## 参考資料

- [Datadog Continuous Profiler](https://docs.datadoghq.com/ja/tracing/profiler/)
- [Datadog Go Profiler](https://docs.datadoghq.com/ja/tracing/profiler/getting_started/?tab=go)
- [Kubernetes上でのDatadog Agent](https://docs.datadoghq.com/ja/agent/kubernetes/?tab=operator)
- [k6負荷テスト](https://k6.io/docs/)

## ライセンス

このプロジェクトはデモ用途のため、商用利用は避けてください。

---

**注意**: このリポジトリはデモ用のため、本番環境での使用は推奨されません。