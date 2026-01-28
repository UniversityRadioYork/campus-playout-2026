def imageTag = ''
def imageName = 'evergiven.ury.york.ac.uk:5000/campus-playout-2026'

pipeline {
    agent {
        node {
            label 'docker'
        }
    }

    stages {
        stage('Build docker image') {
            steps {
                sh 'nix build --show-trace --log-lines 10000 --fallback .#docker'
                sh "./result | docker image load"
            }
        }

        stage('Tag and push docker image') {
            steps {
                sh "docker image tag campus-playout-2026:latest ${imageName}:$GIT_COMMIT"
                sh "docker image push ${imageName}:$GIT_COMMIT"
            }
        }

        stage('Tag and push release docker image') {
            when {
                tag(pattern: /^\d{4}-\d{2}-\d{2}$/, comparator: "REGEXP")
            }

            steps {
                script {
                    imageTag = env.TAG_NAME.replace('v', '')
                    sh "docker image tag campus-playout-2026:latest ${imageName}:${imageTag}"
                    sh "docker image push ${imageName}:${imageTag}"
                }
            }
        }

        stage('Deploy to development') {
            when {
                branch 'trunk'
            }

            steps {
                sh "docker service update --image ${imageName}:$GIT_COMMIT campus-playout-dev"
            }
        }

        stage('Deploy to production') {
            when {
                tag(pattern: /^\d{4}-\d{2}-\d{2}$/, comparator: "REGEXP")
            }

            steps {
                script {
                    sh "docker service update --image ${imageName}:${imageTag} campus-playout-courtyard"
                    sh "docker service update --image ${imageName}:${imageTag} campus-playout-kitchen"
                }
            }
        }
    }
}
