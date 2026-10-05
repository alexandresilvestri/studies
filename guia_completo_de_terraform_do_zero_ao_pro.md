# 🛠️ Guia Completo de Terraform: Do Zero ao Pro
> **Baseado no curso:** *Complete Terraform Course - From BEGINNER to PRO! (DevOps Directive / Sid Palas)*  
> **Objetivo:** Manual prático e teórico completo para provisionamento e automação de infraestrutura em nuvem (AWS) utilizando Terraform e HashiCorp Configuration Language (HCL).

---

## 📌 Sumário
1. [Módulo 1: Evolução da Nuvem e Infraestrutura como Código (IaC)](#módulo-1-evolução-da-nuvem-e-infraestrutura-como-código-iac)
2. [Módulo 2: Visão Geral e Configuração do Ambiente](#módulo-2-visão-geral-e-configuração-do-ambiente)
3. [Módulo 3: Fluxo de Trabalho Básico e Gerenciamento de Estado](#módulo-3-fluxo-de-trabalho-básico-e-gerenciamento-de-estado)
4. [Módulo 4: Variáveis e Saídas (Variables & Outputs)](#módulo-4-variáveis-e-saídas-variables--outputs)
5. [Módulo 5: Recursos Avançados da Linguagem HCL](#módulo-5-recursos-avançados-da-linguagem-hcl)
6. [Módulo 6: Organização e Módulos no Terraform](#módulo-6-organização-e-módulos-no-terraform)
7. [Módulo 7: Gerenciamento de Múltiplos Ambientes](#módulo-7-gerenciamento-de-múltiplos-ambientes)
8. [Módulo 8: Testes em Infraestrutura como Código](#módulo-8-testes-em-infraestrutura-como-código)
9. [Módulo 9: Fluxo de Trabalho em Equipe e CI/CD com GitHub Actions](#módulo-9-fluxo-de-trabalho-em-equipe-e-cicd-com-github-actions)
10. [Resumo das Melhores Práticas](#resumo-das-melhores-práticas)

---

## Módulo 1: Evolução da Nuvem e Infraestrutura como Código (IaC)

### 1.1 Evolução do Servidor Físico ao Serverless
* **Bare Metal (Servidores Físicos):** Configuração manual, lentidão no provisionamento, desperdício de recursos.
* **Virtualização e Máquinas Virtuais (VMs):** Abstração do hardware via Hypervisor. Início da automação de nuvem (AWS EC2, GCP Compute Engine).
* **Contêineres (Docker, K8s):** Isolamento em nível de sistema operacional, inicialização em segundos e alta densidade.
* **Funções Serverless / PaaS:** Abstração total do servidor infraestrutural (AWS Lambda, Fargate).

### 1.2 O que é Infraestrutura como Código (IaC)?
IaC é a prática de gerenciar e provisionar infraestrutura de TI por meio de arquivos de definição legíveis por máquina, em vez de processos manuais no painel da nuvem.

#### Benefícios Principais:
* **Rastreabilidade e Versionamento:** Armazenado em controle de versão (Git).
* **Repetibilidade e Consistência:** Elimina o "Configuration Drift" (desvio de configuração).
* **Automação e Agilidade:** Criação de ambientes em minutos.
* **Documentação Viva:** O código expressa exatamente o que está implantado.

### 1.3 Abordagem Declarativa vs. Imperativa
* **Declarativa (Terraform, CloudFormation):** Você define o **estado final desejado** do sistema. A ferramenta calcula as diferenças e aplica as mudanças necessárias.
* **Imperativa (Ansible, Scripts Bash/Python):** Você especifica os **passos exatos** a serem executados em ordem cronológica.

---

## Módulo 2: Visão Geral e Configuração do Ambiente

### 2.1 Requisitos Iniciais
1. **Terraform CLI:** Instalação via gerenciador de pacotes (`brew`, `apt`, `choco`).
2. **AWS CLI:** Configurado com permissões do IAM (`aws configure`).
3. **Editor de Código:** VS Code com extensão oficial do Terraform/HCL.

### 2.2 Primeiro Exemplo Prático (`main.tf`)
Criando um recurso básico de instância EC2 na AWS:

```hcl
terraform {
  required_version = ">= 1.0.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = "us-east-1"
}

resource "aws_instance" "web_server" {
  ami           = "ami-0c55b159cbfafe1f0" # Amazon Linux 2 (depende da região)
  instance_type = "t2.micro"

  tags = {
    Name = "PrimeiroServidorTerraform"
  }
}
```

---

## Módulo 3: Fluxo de Trabalho Básico e Gerenciamento de Estado

### 3.1 O Ciclo de Vida do Terraform (Core Workflow)

```
 [ Ecrever Código HCL ]
          │
          ▼
    terraform init      ──► Baixa Provedores e Módulos
          │
          ▼
    terraform plan      ──► Compara Estado Atual vs. Desejado
          │
          ▼
    terraform apply     ──► Aplica as alterações na Nuvem
          │
          ▼
    terraform destroy   ──► Destrói a Infraestrutura
```

#### Detalhamento dos Comandos:
* `terraform init`: Inicializa o diretório de trabalho, baixa os plugins dos *providers*.
* `terraform fmt`: Formata automaticamente os arquivos `.tf` no padrão HCL.
* `terraform validate`: Checa a sintaxe e a coerência do código.
* `terraform plan`: Gera um plano de execução detalhado (adições `+`, alterações `~`, destruições `-`).
* `terraform apply`: Executa as alterações previstas. Suporta a flag `-auto-approve`.
* `terraform destroy`: Remove todos os recursos gerenciados pelo estado atual.

### 3.2 O Arquivo de Estado (`terraform.tfstate`)
* O Terraform utiliza o arquivo `terraform.tfstate` para mapear recursos declarados no HCL com os objetos reais criados no provedor de nuvem.
* **Importante:** O arquivo de estado contém informações sensíveis (como senhas e chaves privadas) e **nunca deve ser commitado no Git**.

### 3.3 Configurando um Backend Remoto (Remote State + State Locking)
Para trabalho em equipe, o estado deve residir em um local centralizado com suporte a bloqueio de estado (*State Locking*) para evitar concorrência.

Exemplo usando **AWS S3** e **DynamoDB**:

```hcl
terraform {
  backend "s3" {
    bucket         = "meu-bucket-de-estado-terraform"
    key            = "global/s3/terraform.tfstate"
    region         = "us-east-1"
    dynamodb_table = "terraform-locks"
    encrypt        = true
  }
}
```

---

## Módulo 4: Variáveis e Saídas (Variables & Outputs)

### 4.1 Variáveis de Entrada (`variables.tf`)
Permitem parametrizar e reutilizar o código sem alterar o manifesto principal.

```hcl
variable "instance_type" {
  description = "Tipo da instância EC2"
  type        = string
  default     = "t2.micro"
}

variable "server_port" {
  description = "Porta HTTP para o servidor web"
  type        = number
  default     = 8080
}

variable "enable_monitoring" {
  description = "Habilitar monitoramento detalhado"
  type        = bool
  default     = false
}

variable "db_password" {
  description = "Senha do banco de dados"
  type        = string
  sensitive   = true # Impede exibição nos logs e terminal
}
```

### 4.2 Formas de Definir Valores para Variáveis
1. **Padrão no código (`default`):** Definido no bloco da variável.
2. **Flag na CLI:** `terraform apply -var="instance_type=t3.micro"`
3. **Arquivo de Variáveis (`terraform.tfvars` ou `*.auto.tfvars`):**
   ```hcl
   instance_type = "t2.small"
   server_port   = 80
   ```
4. **Variáveis de Ambiente:** Exportando no terminal:
   ```bash
   export TF_VAR_db_password="MinhaSenhaSegura123"
   ```

### 4.3 Variáveis de Saída (`outputs.tf`)
Exibem informações úteis no terminal ou exportam dados para outros módulos.

```hcl
output "public_ip" {
  description = "Endereço IP público da instância"
  value       = aws_instance.web_server.public_ip
}

output "db_endpoint" {
  description = "Endpoint de conexão do banco de dados"
  value       = aws_db_instance.database.endpoint
}
```

---

## Módulo 5: Recursos Avançados da Linguagem HCL

### 5.1 Fontes de Dados (*Data Sources*)
Permitem consultar recursos existentes na nuvem que não foram criados pela configuração atual do Terraform.

```hcl
# Busca a AMI oficial mais recente do Ubuntu
data "aws_ami" "ubuntu" {
  most_recent = true
  owners      = ["099720109477"] # Canonical

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
  }
}

resource "aws_instance" "app" {
  ami           = data.aws_ami.ubuntu.id
  instance_type = "t2.micro"
}
```

### 5.2 Meta-Argumentos de Controle
* **`count`:** Cria múltiplas instâncias idênticas com base em um número.
  ```hcl
  resource "aws_instance" "server" {
    count         = 3
    ami           = "ami-0c55b159cbfafe1f0"
    instance_type = "t2.micro"

    tags = {
      Name = "Server-${count.index}"
    }
  }
  ```

* **`for_each`:** Itera sobre conjuntos de dados (*sets* ou *maps*).
  ```hcl
  variable "user_names" {
    type    = set(string)
    default = ["alice", "bob", "charlie"]
  }

  resource "aws_iam_user" "users" {
    for_each = var.user_names
    name     = each.value
  }
  ```

* **`depends_on`:** Força uma dependência explícita entre recursos.
* **`lifecycle`:** Controla ações de substituição de recursos:
  ```hcl
  lifecycle {
    create_before_destroy = true
    prevent_destroy       = true
    ignore_changes        = [tags]
  }
  ```

### 5.3 Bloco Dinâmico (`dynamic`)
Gera blocos repetitivos internamente dentro de um recurso (ex: regras de Security Group).

```hcl
variable "ingress_ports" {
  type    = list(number)
  default = [80, 443, 8080]
}

resource "aws_security_group" "web_sg" {
  name = "sg-web"

  dynamic "ingress" {
    for_each = var.ingress_ports
    content {
      from_port   = ingress.value
      to_port     = ingress.value
      protocol    = "tcp"
      cidr_blocks = ["0.0.0.0/0"]
    }
  }
}
```

---

## Módulo 6: Organização e Módulos no Terraform

### 6.1 O que são Módulos e Por que Usá-los?
Um módulo é um conjunto de arquivos `.tf` organizados em um único diretório. Eles promovem:
* Reutilização de código (**DRY - Don't Repeat Yourself**).
* Encapsulamento e abstração de infraestruturas complexas.
* Padronização de arquitetura na organização.

### 6.2 Estrutura de Diretórios de um Módulo

```
modules/
└── aws-web-app/
    ├── main.tf        # Recursos do módulo
    ├── variables.tf   # Entradas do módulo
    ├── outputs.tf     # Saídas do módulo
    └── README.md      # Documentação
```

### 6.3 Consumindo um Módulo Customizado

```hcl
module "production_web_app" {
  source = "./modules/aws-web-app"

  instance_type = "t3.medium"
  environment   = "production"
  db_name       = "proddb"
}
```

---

## Módulo 7: Gerenciamento de Múltiplos Ambientes

O curso aborda as duas estratégias principais para separar ambientes (ex: `dev`, `staging`, `prod`):

| Estratégia | Como funciona | Vantagens | Desvantagens |
| :--- | :--- | :--- | :--- |
| **Workspaces** | Um único conjunto de código com múltiplos arquivos de estado isolados na CLI. | Fácil chaveamento (`terraform workspace select dev`). | Menos visível; pode ocasionar erros acidentais de destruição de produção. |
| **Diretórios Separados** | Diretórios independentes (`env/dev/`, `env/prod/`) que chamam os mesmos módulos. | Isolamento total de credenciais e estados; controle claro via Git. | Duplicação leve de chamadas de módulos. |

### Exemplo Recomendado: Estrutura por Diretórios

```
.
├── modules/
│   └── web_app/
└── environments/
    ├── dev/
    │   ├── main.tf       # Invoca ../../modules/web_app com parâmetros de DEV
    │   └── backend.tf
    └── prod/
        ├── main.tf       # Invoca ../../modules/web_app com parâmetros de PROD
        └── backend.tf
```

---

## Módulo 8: Testes em Infraestrutura como Código

### 8.1 Ferramentas de Validação e Qualidade
1. **`terraform fmt -check`:** Verifica a formatação do código no CI.
2. **`terraform validate`:** Garante consistência sintática do HCL.
3. **`TFLint`:** Análise estática focada em erros específicos do provedor (ex: nomes de instâncias EC2 inválidos).
4. **`Checkov` / `tfsec`:** Análise estática de segurança e conformidade (ex: buckets S3 abertos ao público).

### 8.2 Testes Automatizados de Integração (Terratest)
Testes em linguagem **Go** que provisionam a infraestrutura real, executam asserções de teste e destroem os recursos em seguida.

```go
package test

import (
	"testing"
	"github.com/gruntwork-io/terratest/modules/terraform"
	"github.com/stretchr/testify/assert"
)

func TestTerraformWebApp(t *testing.T) {
	opts := &terraform.Options{
		TerraformDir: "../environments/dev",
	}

	defer terraform.Destroy(t, opts)
	terraform.InitAndApply(t, opts)

	publicIp := terraform.Output(t, opts, "public_ip")
	assert.NotEmpty(t, publicIp)
}
```

---

## Módulo 9: Fluxo de Trabalho em Equipe e CI/CD com GitHub Actions

### 9.1 Fluxo Recomendado de Colaboração (PR-driven)
1. O desenvolvedor cria uma *feature branch* e altera o código HCL.
2. Abre um **Pull Request (PR)** para a branch principal.
3. O pipeline de CI executa `terraform fmt`, `validate` e gera um `terraform plan` postado como comentário no PR.
4. Após revisão e merge na branch `main`, a esteira executa o `terraform apply`.

### 9.2 Exemplo de Pipeline no GitHub Actions (`.github/workflows/terraform.yml`)

```yaml
name: "Terraform CI/CD"

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:
  terraform:
    name: "Terraform Execution"
    runs-on: ubuntu-latest

    steps:
    - name: Checkout Repository
      uses: actions/checkout@v3

    - name: Setup Terraform
      uses: hashicorp/setup-terraform@v2
      with:
        terraform_version: 1.5.0

    - name: Configure AWS Credentials
      uses: aws-actions/configure-aws-credentials@v2
      with:
        aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY_ID }}
        aws-secret-access-key: ${{ secrets.AWS_SECRET_ACCESS_KEY }}
        aws-region: us-east-1

    - name: Terraform Init
      run: terraform init

    - name: Terraform Format Check
      run: terraform fmt -check

    - name: Terraform Plan
      if: github.event_name == 'pull_request'
      run: terraform plan -no-color

    - name: Terraform Apply
      if: github.ref == 'refs/heads/main' && github.event_name == 'push'
      run: terraform apply -auto-approve
```

---

## Resumo das Melhores Práticas

1. **Proteja o Estado (`.tfstate`):** Utilize sempre backends remotos com *State Locking* e criptografia ativada.
2. **Nunca Coloque Segredos no Código:** Utilize variáveis marcadas como `sensitive = true`, Cofres de Segredos (AWS Secrets Manager/HashiCorp Vault) ou variáveis de ambiente.
3. **Mantenha o Código Modular:** Abstraia estruturas repetitivas em módulos reutilizáveis.
4. **Isole Ambientes:** Dê preferência à separação por diretórios para isolar completamente o estado de desenvolvimento do de produção.
5. **Automação de CI/CD:** Não execute `apply` manualmente na sua máquina local para ambientes de produção; utilize pipelines controladas com code review.