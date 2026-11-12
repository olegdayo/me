use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn points(items: &[Html]) -> Html {
    html! { <ul>{for items.iter().map(|item| html! { <li>{item.clone()}</li> })}</ul> }
}

fn group(title: &str, items: &[Html]) -> Html {
    html! { <div class="cv-group"><h3>{title}</h3>{points(items)}</div> }
}

#[function_component(CV)]
pub fn cv() -> Html {
    html! {
        <div class="general">
            <nav class="back-row cv-actions" aria-label="CV navigation">
                <Link<Route> to={Route::General}>{"← Back"}</Link<Route>>
                <a class="pdf-download" href="assets/images/CV.pdf" download="OlegSidorenkovCV.pdf">
                    <img src="assets/images/pdf.png" alt="" width="32" height="32" />
                    <span>{"Export as PDF"}</span>
                </a>
            </nav>
            <article class="cv">
                <div class="cv-column">
                <header class="cv-header">
                    <div class="profile-photo-frame">
                        <img class="profile-photo" src="assets/images/profile.png" alt="Oleg Sidorenkov" />
                    </div>
                    <div class="profile-details">
                    <h1>{"Oleg Sidorenkov"}</h1>
                    <p class="role">{"Senior SWE / Tech Lead"}</p>
                    <address class="contacts">
                        <a href="tel:+79037774240">{"+7 (903) 777-42-40"}</a>
                        <a href="mailto:ol-sidorenkov@mail.ru">{"ol-sidorenkov@mail.ru"}</a>
                        <a href="https://t.me/olegdayo">{"t.me/olegdayo"}</a>
                        <a href="https://www.linkedin.com/in/olegdayo/">{"linkedin.com/in/olegdayo"}</a>
                        <a href="https://github.com/olegdayo">{"github.com/olegdayo"}</a>
                    </address>
                    </div>
                </header>

                <section>
                    <h2>{"Summary"}</h2>
                    <p><strong>{"Senior Software Engineer, 4+ years of experience"}</strong></p>
                    {points(&[
                        html! { {"Backend, large-scale distributed systems, security, and cloud"} },
                        html! { {"Go, PostgreSQL, Kafka, Linux, Kubernetes, Vault, Terraform"} },
                    ])}
                    <p><strong>{"Tech Lead, KMS Infrastructure, 1+ years of experience"}</strong></p>
                    {points(&[
                        html! { {"E2E ownership and cross-team delivery of platform products"} },
                        html! { {"Mentoring and hiring engineers (5+ hires, 50+ interviews)"} },
                        html! { {"Security Champion; driving DevOps practices within the team"} },
                        html! { <a href="https://github.com/olegdayo/talks">{"Public speaker at IT conferences; tech writer; open-source contributor"}</a> },
                    ])}
                </section>

                <section>
                    <h2>{"Experience"}</h2>
                    <div class="cv-entry">
                        <div class="entry-heading"><h3>{"VK Cloud Senior SWE / Tech Lead"}</h3><span>{"2025.04–present"}</span></div>
                        <p>{"KMS (PaaS), Senior SWE → KMS (PaaS) Tech Lead"}</p>
                        {group("Architecture & Leadership", &[
                            html! { {"Designed a multi-datacenter architecture for a cloud KMS based on HashiCorp Vault/OpenBao + designed API. 5 data-centers, 4 envs, both bare-metal and K8s"} },
                            html! { {"Delivered KMS and S3 teams integration in bucket encryption: gathered requirements, created ADR, built environments, and launched MVP"} },
                            html! { {"Delivered KMS and K8s teams integration in ETCD encryption: gathered requirements, created multiple ADRs/MVPs, and launched into production"} },
                            html! { {"Conducted over 20 interviews. Hired 3 Middle/Senior Go devs for PaaS teams"} },
                            html! { {"Reduced bus factor through architecture documentation, duty handbooks, shared technical ownership, and mentoring engineers"} },
                        ])}
                        {group("Development", &[
                            html! { {"Led the design and implementation of a Control-Plane that automates Vault cluster lifecycle and configuration: initialisation, unsealing, provisioning of various entities (plugins, secrets, keys, users, policies, etc...)"} },
                            html! { <a href="https://github.com/vk-cs/terraform-provider-vkcs/pull/723">{"Created a Terraform provider for managing lifecycle of secrets/keys in the cloud"}</a> },
                            html! { {"Built a KMS Data-Plane exposed as VK Cloud KMS secrets/keys API"} },
                            html! { <a href="https://github.com/FalcoSuessgott/vault-kubernetes-kms/pull/294">{"Forked and modified a K8s plugin for encrypting resources stored in ETCD"}</a> },
                            html! { {"Improved pods CPU usage across services by ~15% and memory usage by ~25% via pprof research, tuned token types and lifetimes, removed redundant projects"} },
                        ])}
                        {group("Operations & Security", &[
                            html! { {"Integrated ETCD encryption into Managed Kubernetes cluster based on Vault and K8s plugins"} },
                            html! { <>{"Created "}<a href="https://github.com/vk-cs/terraform-vkcs-kms">{"Terraform modules"}</a>{" for easier secrets migration from other clouds"}</> },
                            html! { {"Owned delivery, security, and observability of KMS services:"} },
                        ])}
                        {points(&[
                            html! { {"Built and hardened rootless distroless multistage Docker images"} },
                            html! { {"Implemented GitLab CI/CD pipelines: Building, Testing, Linting, Scanning vulnerabilities/CVEs, and Delivery to K8s clusters via HelmWave"} },
                            html! { {"Set up monitoring: metrics and alerting (VictoriaMetrics + AlertManager)"} },
                            html! { {"Delivered Helm charts for backend services"} },
                            html! { {"Fixed over 100 vulnerabilities across services (misconfigs, deps, secrets, RBAC)"} },
                            html! { {"Duty: Incident (on-call) and operations (asynchronous help in chat)"} },
                        ])}
                    </div>
                    <div class="cv-entry">
                        <div class="entry-heading"><h3>{"Ozon SWE"}</h3><span>{"2023.11–2024.12"}</span></div>
                        <p>{"Seller’s Actions (FinTech), Middle SWE"}</p>
                        {group("Architecture & Leadership", &[
                            html! { {"Redesigned PostgreSQL sharding strategy. Average number of live connections dropped from 50 to 30. Largest table row count reduced by 90%"} },
                            html! { {"Coordinated 2 teams (analytics and development) during Kafka events API migration. The change resulted in reducing latency by 50% in p50, by 80% in p90, and by 90% in p99"} },
                            html! { {"Conducted over 30 technical interviews. Hired 3 Go devs for our team"} },
                        ])}
                        {group("Development", &[
                            html! { {"Maintained Go-microservices product serving 5,000+ RPS"} },
                            html! { {"Built an API that allows sellers to communicate with the system via Excel"} },
                            html! { {"Migrated PostgreSQL (re-implemented all queues, implemented migrator)"} },
                            html! { {"Reduced architectural technical debt and removed legacy components"} },
                        ])}
                        {group("Operations", &[
                            html! { {"Created GitLab CI pipelines such as ProtoLint static check"} },
                            html! { {"Duty: Incident (on-call) and operations (asynchronous help in chat)"} },
                        ])}
                    </div>
                    <div class="cv-entry">
                        <div class="entry-heading"><h3>{"Yandex SRE / SWE"}</h3><span>{"2022.07–2023.11"}</span></div>
                        <p>{"NOC, Intern SWE → Market, Junior SRE → Market, Middle SRE"}</p>
                        {group("Development", &[
                            html! { {"Maintained a warehouse service, that monitors states of CUPs/NUCs/PDTs. Go, PostgreSQL, ReactTS"} },
                            html! { {"Maintained firewall config service. Go, Python, React, MongoDB. Led the integration with Yandex.Calendar API"} },
                        ])}
                        {group("Operations", &[
                            html! { {"Set up CI/CD pipelines and monitoring"} },
                            html! { {"Duty: Incident (on-call) and operations (asynchronous help in chat)"} },
                        ])}
                    </div>
                </section>

                </div>
                <div class="cv-column">
                <section>
                    <h2>{"Skills"}</h2>
                    {group("Development", &[
                        html! { {"Go (Backend, distributed systems, CLIs, automation, pprof)"} },
                        html! { {"Rust, C, Shell/Bash, Python"} },
                        html! { {"Databases (PostgreSQL, ClickHouse, Redis, MongoDB, ETCD, S3)"} },
                        html! { {"Queues/Event Streaming (Kafka)"} },
                    ])}
                    {group("Operations", &[
                        html! { {"Linux (administration, containerisation, networking, ptrace, strace, perf)"} },
                        html! { {"Kubernetes (Helm, CRD, CRI/CNI/CSI, Docker images)"} },
                        html! { {"Networking (TCP/IP, HTTP, gRPC, NetBox, eBPF, DPDK)"} },
                        html! { {"CI/CD (GitHub Actions, GitLab CI/CD, Jenkins, ArgoCD)"} },
                        html! { {"IaC (Terraform, Ansible)"} },
                        html! { {"Cloud (OpenStack, VK Cloud, Yandex Cloud)"} },
                    ])}
                    {group("Observability", &[
                        html! { {"Metrics (Prometheus, VictoriaMetrics, Grafana)"} },
                        html! { {"Logging (Zap, OpenTelemetry, ELKB)"} },
                        html! { {"Tracing (OpenTelemetry, Jaeger)"} },
                        html! { {"Alerting (AlertManager, Sentry)"} },
                    ])}
                    {group("System Design & Architecture", &[
                        html! { {"API (OpenAPI/Swagger, protobuf)"} },
                        html! { {"High Availability & High Load (CAP, caching, L4/L7 load balancing, profiling)"} },
                        html! { {"Fault Tolerance (DRP, graceful shutdown, graceful degradation, backoff)"} },
                        html! { {"Microservices & Async Intercommunication"} },
                    ])}
                    {group("Security", &[
                        html! { {"Keys and secrets (HashiCorp Vault, OpenStack Barbican)"} },
                        html! { {"K8s (RBAC, Kyverno, image hardening, non-root, distroless)"} },
                        html! { {"Scanners (Trivy, Linters)"} },
                    ])}
                    {group("Testing", &[
                        html! { {"Unit/Fuzzing (mocks)"} },
                        html! { {"Integration/Acceptance/E2E (TestContainers, Terraform)"} },
                    ])}
                </section>

                <section>
                    <h2>{"Education & Certifications"}</h2>
                    {group("Education", &[
                        html! { {"PhD student (part-time), Economics, 2024–present"} },
                        html! { <a href="https://www.hse.ru/en/ma/se">{"Master’s degree, Computer Science, HSE FCS, 2024–2026, GPA 8.1/10"}</a> },
                        html! { <a href="https://www.hse.ru/en/ba/se">{"Bachelor’s degree, Computer Science, HSE FCS, 2020–2024, GPA 8.65/10"}</a> },
                        html! { <a href="https://fiw.thws.de/en">{"External student for 1 semester at THWS, 2023"}</a> },
                    ])}
                    {group("Certifications", &[html! { <a href="https://drive.google.com/file/d/1dNdwlJjCvLXMYr3e4lWZTAZ60QOuLenm/view?usp=sharing">{"Tinkoff SRE"}</a> }])}
                </section>

                <section>
                    <h2>{"Talks & Publications"}</h2>
                    {group("Speaker", &[
                        html! { <a href="https://youtu.be/heXa5PnY200">{"“Contextual Talk”, Evrone Go Meetup"}</a> },
                        html! { <a href="https://ecode.ozon.tech/talks/b14777a889154860ac8b580aea73ffe8">{"“Obscure STL Containers and Where to Apply Them”, E-CODE"}</a> },
                        html! { <a href="https://drive.google.com/file/d/12XRe6Y8sv7XfbMExBvm-UOjAb4VBFNdU/view?usp=sharing">{"“Container Internals or how 4 Syscalls Changed the Operations”, Stachka"}</a> },
                    ])}
                    {group("Writer", &[html! { <a href="https://habr.com/ru/companies/vk/articles/1004422/">{"“Container Internals or how 4 Syscalls Changed the Operations”, Habr"}</a> }])}
                </section>

                <section>
                    <h2>{"Open-source & Pet-projects"}</h2>
                    <h3><a href="https://github.com/vk-cs/terraform-provider-vkcs/pull/723">{"Terraform Provider VKCS"}</a></h3>
                    <p>{"Terraform provider for managing VK Cloud entities. I’ve implemented KMS resources and data sources as well as provided "}<a href="https://github.com/vk-cs/terraform-vkcs-kms">{"modules (e.g. migration from other clouds)"}</a></p>
                    <p class="stack">{"Stack: Go, HashiCorp Vault, HashiCorp Terraform"}</p>
                    <h3><a href="https://github.com/FalcoSuessgott/vault-kubernetes-kms/pull/294">{"Vault Kubernetes KMS"}</a></h3>
                    <p>{"K8s Control Plane component that encrypts Secrets and ConfigMaps via Vault. Thus, entities stored in ETCD are secured. I’ve implemented userpass auth integration"}</p>
                    <p class="stack">{"Stack: Go, Kubernetes, HashiCorp Vault"}</p>
                    <h3><a href="https://github.com/2Delight/devsecmlops-template">{"DevSecMLOpsTemplate"}</a></h3>
                    <p>{"Template for secure deployment of ML services based on CRDs/Terraform entities for integrating with trusted model registry and Kyverno policies over S3 and Vault"}</p>
                    <p class="stack">{"Stack: Go, K8s, CRD, Kyverno, Helm, GitLab CI/CD, AWS S3, ArgoCD, Terraform, Vault"}</p>
                    <h3>{"Go-libs"}</h3>
                    <p><a href="https://github.com/olegdayo/omniconv">{"Declarative type conversion"}</a>{", "}<a href="https://github.com/olegdayo/slru">{"LRU-cache"}</a></p>
                    <p class="stack">{"Stack: Go"}</p>
                    <h3><a href="https://github.com/2Delight/mascoty-taurine">{"Mascoty"}</a></h3>
                    <p>{"Cross-platform application for creating 2D-avatars with real-time facial expressions"}</p>
                    <p class="stack">{"Stack: Rust, TypeScript, Tauri, Torch"}</p>
                </section>

                <section>
                    <h2>{"Teaching"}</h2>
                    <div class="entry-heading"><h3>{"HSE Instructor"}</h3><span>{"2025.01–present"}</span></div>
                    <p><a href="https://www.hse.ru/ba/se/courses/966769334.html">{"Operating Systems"}</a>{" course instructor. I conduct classes about UNIX/Linux internals"}</p>
                    <div class="entry-heading"><h3>{"HSE & School №444 Teaching Assistant"}</h3><span>{"2021.09–2022.05"}</span></div>
                    {points(&[
                        html! { <a href="https://cs.hse.ru/initiative/2021/2022-2">{"Algebra and Calculus Mathematical Practice in Matlab teaching assistant"}</a> },
                        html! { {"Data Structures and Algorithms in Python teacher"} },
                    ])}
                </section>

                <section>
                    <h2>{"Languages"}</h2>
                    {points(&[html! { {"🇷🇺 Russian, native"} }, html! { <>{"🇬🇧 English, C1, "}<a href="https://drive.google.com/file/d/1k4lelwvUKgJsic1ozDnYsN4PFMYLvqde/view?usp=sharing">{"Academic IELTS – 7.5/9"}</a></> }])}
                </section>
                </div>
            </article>
        </div>
    }
}
