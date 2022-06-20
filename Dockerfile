# test runner image
FROM ghcr.io/james-chf/devchain-container:sha-8e60590
RUN groupadd -g 1000 testrunner && \
    useradd -r -m -u 1000 -g testrunner testrunner

RUN rm -rf wasm/
COPY build/debug/ wasm/
COPY build/tests/ tests/

RUN chown -R testrunner:testrunner .
USER testrunner